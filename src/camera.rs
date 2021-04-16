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
    pub fn new(lookfrom: vec3::Point3, lookat: vec3::Point3, vup: vec3::Vec3,
               vfov: f32, aspect_ratio: f32) -> Self {
        let theta: f32 = util::degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w;

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
