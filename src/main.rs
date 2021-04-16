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

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let canvas = Canvas::new(image_width, image_height)
        .title("Tile")
        .render_on_change(true);
    let mut ppm_writer = match PpmWriter::new(&image_width, &image_height) {
        Err(why) => panic!("couldn't create writer {}", why),
        Ok(ppm_writer) => ppm_writer,
    };

    // Materials
    let material_ground = Rc::new(material::Lambertian::new(color::Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Lambertian::new(color::Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(material::Dielectric::new(1.5));
    let material_right = Rc::new(material::Metal::new(color::Color::new(0.8, 0.6, 0.2), 0.0));

    // World
    let mut world: HittableList = HittableList::new(Box::new(sphere::Sphere::new(
        vec3::Point3(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3(1.0, -0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

    let camera = camera::Camera::new();

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
