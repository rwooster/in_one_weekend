#![allow(dead_code)]

mod color;
mod ppm;
mod ray;
mod vec3;

use ppm::PpmWriter;
use std::ops;

// Scale values from start -> end for 0 <= t <= 1.
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
    let t = hit_sphere(vec3::Point3(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        // Get a surface normal - a vector perpendicular to the surface at the intersection point.
        let N = (r.at(t) - vec3::Vec3(0.0, 0.0, -1.0)).unit_vector();

        // We map x/y/z of N to r/g/b for easy visualization.
        return color::Color(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) * 0.5;
    }

    // Gradient white -> vlue background.
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    linear_blend(&color::WHITE, &color::BLUE, t)
}

// Check if the given sphere is hit by the ray.
// If so, returns the value t along the ray which touches the sphere.
fn hit_sphere(center: vec3::Point3, radius: f32, r: &ray::Ray) -> f32 {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = oc.dot(r.direction) * 2.0;
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;

    // Check if there is a solution to the quadratic - if so, calculate t and return it.
    // We only look at the closest hit point if there are multiple.
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (a * 2.0)
    }
}

fn main() -> std::io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

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

    // Render
    let mut ppm_writer = match PpmWriter::new(&image_width, &image_height) {
        Err(why) => panic!("couldn't create writer {}", why),
        Ok(ppm_writer) => ppm_writer,
    };

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
            ppm_writer.write_color(pixel_color)?;
        }
    }
    eprintln!("");

    Ok(())
}
