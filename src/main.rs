#![allow(dead_code)]
#![allow(unused_variables)]

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ppm;
mod ray;
mod sphere;
mod util;
mod vec3;

use hittable::Hittable;
use hittable_list::HittableList;
use pixel_canvas::{Canvas, Color, RC};
use ppm::PpmWriter;
use std::ops;
use std::rc::Rc;

// t == 0, returns start, t == 1 returns end.
fn linear_blend<T>(start: &T, end: &T, t: f32) -> T
where
    T: ops::Mul<f32, Output = T> + ops::Add<Output = T> + Copy,
{
    *start * (1.0 - t) + (*end * t)
}

// Given a ray from camera -> pixel in the image, determine the color of that pixel.
fn ray_color(r: &ray::Ray, world: &HittableList, depth: usize) -> color::Color {
    if depth <= 0 {
        return color::Color(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.0001, f32::INFINITY) {
        Some(hit_record) => match hit_record.material.scatter(r, &hit_record) {
            Some(scattering) => {
                return ray_color(&scattering.scattered, world, depth - 1) * scattering.attenuation;
            }
            None => {
                return color::Color(0.0, 0.0, 0.0);
            }
        },
        None => {
            // Gradient white -> vlue background.
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            return linear_blend(&color::WHITE, &color::BLUE, t);
        }
    }
}

// TODO: Argument parsing
static WRITE_PPM: bool = true;

fn write_pixel(
    pixel_color: &color::Color,
    samples_per_pixel: i32,
    pixel: &mut pixel_canvas::Color,
    ppm_writer: &mut PpmWriter,
) {
    let mut r = pixel_color.0;
    let mut g = pixel_color.1;
    let mut b = pixel_color.2;

    let scale = 1.0 / samples_per_pixel as f32;
    // Gamma correct for gamma=2
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    r = 256.0 * r.clamp(0.0, 0.999);
    g = 256.0 * g.clamp(0.0, 0.999);
    b = 256.0 * b.clamp(0.0, 0.999);

    *pixel = Color {
        r: r as u8,
        g: g as u8,
        b: b as u8,
    };

    if WRITE_PPM {
        ppm_writer
            .write_color(color::Color(r, g, b))
            .expect("writing color failed");
    }
}

fn random_scene() -> HittableList {
    let material_ground = Rc::new(material::Lambertian::new(color::Color::new(0.5, 0.5, 0.5)));

    let mut world: HittableList = HittableList::new(Box::new(sphere::Sphere::new(
        vec3::Point3(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = util::random_float();
            let center = vec3::Point3(
                (a as f32) + 0.9 * util::random_float(),
                0.2,
                (b as f32) + 0.9 * util::random_float(),
            );

            if (center - vec3::Point3(4.0, 0.2, 0.0)).norm() <= 0.9 {
                continue;
            }

            let material: Rc<dyn material::Material> = if choose_material < 0.8 {
                let albedo = color::Color::random() * color::Color::random();
                Rc::new(material::Lambertian::new(albedo))
            } else if choose_material < 0.95 {
                let albedo = color::Color::random_range(0.5, 1.0);
                let fuzz = util::random_float_bounds(0.0, 0.5);
                Rc::new(material::Metal::new(albedo, fuzz))
            } else {
                Rc::new(material::Dielectric::new(1.5))
            };
            world.add(Box::new(sphere::Sphere::new(center, 0.2, material.clone())));
        }
    }

    let material1 = Rc::new(material::Dielectric::new(1.5));
    world.add(Box::new(sphere::Sphere::new(vec3::Point3(0.0, 1.0, 0.0), 1.0, material1.clone())));

    let material2 = Rc::new(material::Lambertian::new(color::Color(0.4, 0.2, 0.1)));
    world.add(Box::new(sphere::Sphere::new(vec3::Point3(-4.0, 1.0, 0.0), 1.0, material2.clone())));

    let material3 = Rc::new(material::Metal::new(color::Color(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(sphere::Sphere::new(vec3::Point3(4.0, 1.0, 0.0), 1.0, material3.clone())));


    return world;
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 3.0 / 2.0;
    let image_width: usize = 1200;
    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let canvas = Canvas::new(image_width, image_height)
        .title("Tile")
        .render_on_change(true);
    let mut ppm_writer = match PpmWriter::new(&image_width, &image_height) {
        Err(why) => panic!("couldn't create writer {}", why),
        Ok(ppm_writer) => ppm_writer,
    };

    let world = random_scene();

    let lookfrom = vec3::Point3(13.0, 2.0, 3.0);
    let lookat = vec3::Point3(0.0, 0.0, 0.0);
    let vup = vec3::Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    canvas.render(move |_state, image| {
        for j in (0..image_height).rev() {
            eprint!("\rScanlines remaining: {}    ", j);
            for i in 0..image_width {
                let mut pixel_color = color::Color(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = ((i as f32) + util::random_float()) / ((image_width - 1) as f32);
                    let v = ((j as f32) + util::random_float()) / ((image_height - 1) as f32);

                    // Generate ray going from camera origin to the current pixel.
                    let r = camera.generate_ray(u, v);
                    pixel_color = pixel_color + ray_color(&r, &world, max_depth);
                }
                let pixel: &mut Color = &mut image[RC(j, i)];
                write_pixel(&pixel_color, samples_per_pixel, pixel, &mut ppm_writer);
            }
        }
        eprintln!("");
    });
    Ok(())
}
