use super::hittable;
use super::ray;
use std::option::Option;

pub struct HittableList {
    pub objects: Vec<Box<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new(initial_object: Box<dyn hittable::Hittable>) -> Self {
        HittableList {
            objects: vec![initial_object],
        }
    }

    pub fn add(&mut self, object: Box<dyn hittable::Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl hittable::Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord> {
        let mut final_record: Option<hittable::HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            match object.hit(r, t_min, closest_so_far) {
                Some(hit_record) => {
                    closest_so_far = hit_record.t;
                    final_record = Some(hit_record);
                }
                None => (),
            }
        }
        final_record
    }
}
