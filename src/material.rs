use super::color;
use super::hittable;
use super::ray;
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
    albedo: color::Color,
}

impl Lambertian {
    pub fn new(albedo: color::Color) -> Self {
        Lambertian { albedo }
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

        Some(Scattering {
            scattered: ray::Ray::new(hit_record.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: color::Color,
    fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: color::Color, fuzziness: f32) -> Self {
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hittable::HitRecord) -> Option<Scattering> {
        let reflected = vec3::reflect(&ray.direction.unit_vector(), &hit_record.normal);
        let scattered = ray::Ray::new(
            hit_record.p,
            reflected + vec3::random_in_unit_sphere() * self.fuzziness,
        );
        if scattered.direction.dot(hit_record.normal) > 0.0 {
            return Some(Scattering {
                scattered,
                attenuation: self.albedo,
            });
        }
        None
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hittable::HitRecord) -> Option<Scattering> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction.unit_vector();
        let cos_theta = (-(unit_direction).dot(hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();
        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;

        let direction = if cannot_refract {
            vec3::reflect(&unit_direction, &hit_record.normal)
        } else {
            vec3::refract(&unit_direction, &hit_record.normal, refraction_ratio)
        };

        let scattered = ray::Ray::new(hit_record.p, direction);
        Some(Scattering {
            scattered,
            attenuation: color::Color::new(1.0, 1.0, 1.0),
        })
    }
}
