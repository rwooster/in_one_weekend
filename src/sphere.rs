use super::hittable;
use super::vec3;
use super::ray;
use std::option::Option;

pub struct Sphere {
    pub center: vec3::Point3,
    pub radius: f32
}

impl Sphere {
    pub fn new(center: vec3::Point3, radius: f32) -> Self {
        Sphere{center, radius}
    }
}


impl hittable::Hittable for Sphere {

    // Check if the given sphere is hit by the ray.
    // If so, returns the hit record for the intersection.
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord> {
        let oc = r.origin - self.center;

        let a = r.direction.norm_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find nearest root in the acceptable range.
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let hit_point = r.at(root);
        Some(hittable::HitRecord{t: root, p: hit_point, normal: (hit_point - self.center) / self.radius})
    }
}
