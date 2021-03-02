use super::ray;
use super::vec3;

struct HitRecord {
    p: vec3::Point3,
    normal: vec3::Vec3,
    t: f32
}

trait hittable {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32, record: &HitRecord) -> bool;

}
