use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

static mut RNG: Option<SmallRng> = None;

pub fn random_double() -> f64 {
    let mut rng = unsafe {
        if RNG.is_none() {
            RNG = Some(SmallRng::from_entropy());
        }

        RNG.as_mut().unwrap()
    };

    rng.gen_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = unsafe {
        if RNG.is_none() {
            RNG = Some(SmallRng::from_entropy());
        }

        RNG.as_mut().unwrap()
    };

    rng.gen_range(min..max)
}
