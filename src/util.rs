use rand::prelude::*;

fn random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn random_float(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
