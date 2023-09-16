mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
pub mod material;
mod ray;
mod sphere;
mod vec;

pub type Vec3 = vec::Vec<3>;
pub type Point3 = Vec3;
pub type Color = Vec3;

pub use camera::Camera;
pub use color::write_color;
pub use hittable::*;
pub use hittable_list::HittableList;
pub use interval::Interval;
pub use material::Material;
use rand::Rng;
pub use ray::Ray;
pub use sphere::Sphere;

/// Return a random float between 0 and 1 included.
pub fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range((0.)..=1.0)
}
