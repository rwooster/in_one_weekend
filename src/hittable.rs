use super::ray;
use super::vec3;
use std::option::Option;

// A record indicating where a Hittable is intersected.
// t: t along the ray where the intersection occurs.
// p: The point of intersection.
// normal: The surface normal from the intersection point.
pub struct HitRecord {
    pub t: f32,
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
