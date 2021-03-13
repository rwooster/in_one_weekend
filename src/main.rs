#![allow(dead_code)]
#![allow(unused_variables)]

mod camera;
mod color;
mod hittable;
mod hittable_list;
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

// t == 0, returns start, t == 1 returns end.
fn linear_blend<T>(start: &T, end: &T, t: f32) -> T
where
    T: ops::Mul<f32, Output = T> + ops::Add<Output = T> + Copy,
{
    *start * (1.0 - t) + (*end * t)
}

// Given a ray from camera -> pixel in the image, determine the color of that pixel.
fn ray_color(r: &ray::Ray, world: &HittableList) -> color::Color {
    match world.hit(r, 0.0, f32::INFINITY) {
        Some(hit_record) => {
            // We map x/y/z of N to r/g/b for easy visualization.
            return (hit_record.normal + color::Color(1.0, 1.0, 1.0)) * 0.5;
        }
        None => {
            // Gradient white -> vlue background.
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            return linear_blend(&color::WHITE, &color::BLUE, t);
        }
    }
}

// TODO: Argument parsing
static WRITE_PPM: bool = false;

fn write_pixel(
    pixel_color: &color::Color,
    pixel: &mut pixel_canvas::Color,
    ppm_writer: &mut PpmWriter,
) {
    *pixel = Color {
        r: (pixel_color.0 * 255.999) as u8,
        g: (pixel_color.1 * 255.999) as u8,
        b: (pixel_color.2 * 255.999) as u8,
    };
    if WRITE_PPM {
        ppm_writer
            .write_color(*pixel_color)
            .expect("writing color failed");
    }
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;

    let canvas = Canvas::new(image_width, image_height)
        .title("Tile")
        .render_on_change(true);
    let mut ppm_writer = match PpmWriter::new(&image_width, &image_height) {
        Err(why) => panic!("couldn't create writer {}", why),
        Ok(ppm_writer) => ppm_writer,
    };

    // World
    let mut world: HittableList = HittableList::new(Box::new(sphere::Sphere::new(
        vec3::Point3(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3(0.0, -100.5, -1.0),
        100.0,
    )));

    let camera = camera::Camera::new();

    canvas.render(move |_state, image| {
        for j in (0..image_height).rev() {
            eprint!("\rScanlines remaining: {}", j);
            for i in 0..image_width {
                let u = (i as f32) / ((image_width - 1) as f32);
                let v = (j as f32) / ((image_height - 1) as f32);

                // Generate ray going from camera origin to the current pixel.
                let r = camera.generate_ray(u, v);

                let pixel_color = ray_color(&r, &world);
                let pixel: &mut Color = &mut image[RC(j, i)];
                write_pixel(&pixel_color, pixel, &mut ppm_writer);
            }
        }
        eprintln!("");
    });
    Ok(())
}
