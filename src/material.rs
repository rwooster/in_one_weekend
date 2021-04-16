use super::hittable;
use super::ray;
use super::color;
use super::vec3;

#[derive(Debug, Copy, Clone)]
pub struct Scattering {
    pub scattered: ray::Ray,
    pub attenuation: color::Color,
}

pub trait Material {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hittable::HitRecord) -> Option<Scattering>;
}


#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: color::Color
}

impl Lambertian {
    pub fn new(albedo: color::Color) -> Self {
        Lambertian{albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hittable::HitRecord) -> Option<Scattering> {
        // Various types of Lambertian diffuse formulation.
        //let target = hit_record.p + hit_record.normal + vec3::random_unit_vector();
        //let target = hit_record.p + vec3::random_in_hemisphere(&hit_record.normal);

       let mut scatter_direction = hit_record.normal * vec3::random_unit_vector(); 

       if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
       }

       Some(Scattering{scattered: ray::Ray::new(hit_record.p, scatter_direction), 
           attenuation: self.albedo})
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: color::Color
}

impl Metal {
    pub fn new(albedo: color::Color) -> Self {
        Metal{albedo}
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hittable::HitRecord) -> Option<Scattering> {
        let reflected = vec3::reflect(&ray.direction.unit_vector(), &hit_record.normal);
        let scattered = ray::Ray::new(hit_record.p, reflected);
        if scattered.direction.dot(hit_record.normal) > 0.0 {
            return Some(Scattering{scattered, attenuation: self.albedo});
        }
        None
    }
}
