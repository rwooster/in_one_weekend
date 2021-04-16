use super::ray;
use super::util;
use super::vec3;

pub struct Camera {
    origin: vec3::Point3,
    lower_left_corner: vec3::Point3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
}

impl Camera {
    pub fn new(vfov: f32, aspect_ratio: f32) -> Self {
        let theta: f32 = util::degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0; // distance from projection plane to projection point.
                                // Camera is at (0, 0, 0)
                                // Y axis is up, X axis is right, into screen is negative Z.
        let origin = vec3::Point3::new(0.0, 0.0, 0.0);
        let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - vec3::Vec3(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn generate_ray(&self, u: f32, v: f32) -> ray::Ray {
        // Generate ray going from camera origin to the given pixel location.
        ray::Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
