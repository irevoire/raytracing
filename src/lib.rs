mod color;
mod ray;
mod vec;

pub type Vec3 = vec::Vec<3>;
pub type Point3 = Vec3;
pub type Color = Vec3;

pub use color::write_color;
pub use ray::Ray;
