use rand::prelude::*;

pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_float_bounds(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}
static PI: f32 = 3.1415926535897932385;

// Utility Functions

pub fn degrees_to_radians(degrees: f32) -> f32 {
    (degrees * PI) / 180.0
}
