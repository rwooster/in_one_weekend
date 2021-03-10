#![allow(dead_code)]
#![allow(unused_variables)]

mod color;
mod ppm;
mod ray;
mod vec3;
mod sphere;
mod hittable;

use pixel_canvas::{Canvas, Color, RC};
use ppm::PpmWriter;
use std::ops;
use hittable::Hittable;

// t == 0, returns start, t == 1 returns end.
fn linear_blend<T>(start: &T, end: &T, t: f32) -> T
where
    T: ops::Mul<f32, Output = T> + ops::Add<Output = T> + Copy,
{
    *start * (1.0 - t) + (*end * t)
}

// Given a ray from camera -> pixel in the image, determine the color of that pixel.
fn ray_color(r: &ray::Ray) -> color::Color {
    // Make a sphere at (0, 0, -1) with a radius of 0.5.
    let sphere = sphere::Sphere::new(vec3::Point3(0.0, 0.0, -1.0), 0.25);

    match sphere.hit(r, 0.0, 1.0) {
        Some(hit_record) => {
            // We map x/y/z of N to r/g/b for easy visualization.
            return color::Color(hit_record.normal.x() + 1.0, hit_record.normal.y() + 1.0, hit_record.normal.z() + 1.0) * 0.5;
        },
        None => {
            // Gradient white -> vlue background.
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            return linear_blend(&color::WHITE, &color::BLUE, t);
        },
    }
}


// TODO: Argument parsing
static WRITE_PPM: bool = false;

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

    // Camera (source of the rays into the scene).
    let viewport_height = 2.0; // arbitrarity chosen height
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0; // distance from projection plane to projection point.

    // Camera is at (0, 0, 0)
    // Y axis is up, X axis is right, into screen is negative Z.
    let origin = vec3::Point3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - vec3::Vec3(0.0, 0.0, focal_length);

    canvas.render(move |_state, image| {
        for j in (0..image_height).rev() {
            eprint!("\rScanlines remaining: {}", j);
            for i in 0..image_width {
                let u = (i as f32) / ((image_width - 1) as f32);
                let v = (j as f32) / ((image_height - 1) as f32);

                // Generate ray going from camera origin to the current pixel.
                let r = ray::Ray::new(
                    origin,
                    lower_left_corner + horizontal * u + vertical * v - origin,
                );
                let pixel_color = ray_color(&r);
                let pixel: &mut Color = &mut image[RC(j, i)];
                *pixel = Color {
                    r: (pixel_color.0 * 255.999) as u8,
                    g: (pixel_color.1 * 255.999) as u8,
                    b: (pixel_color.2 * 255.999) as u8,
                };
                if WRITE_PPM {
                    ppm_writer
                        .write_color(pixel_color)
                        .expect("writing color failed");
                }
            }
        }
        eprintln!("");
    });
    Ok(())
}
