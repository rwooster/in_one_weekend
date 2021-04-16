use super::material;
use super::ray;
use super::vec3;
use std::option::Option;
use std::rc::Rc;

// A record indicating where a Hittable is intersected.
// t: t along the ray where the intersection occurs.
// p: The point of intersection.
// normal: The surface normal from the intersection point.
pub struct HitRecord {
    pub t: f32,
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub front_face: bool,
    pub material: Rc<dyn material::Material>,
}

impl HitRecord {
    pub fn new(
        t: f32,
        p: vec3::Point3,
        r: &ray::Ray,
        outward_normal: vec3::Vec3,
        material: Rc<dyn material::Material>,
    ) -> Self {
        let front_face: bool = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            t,
            p,
            normal,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
