use rand::prelude::*;

pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_float_bounds(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}
