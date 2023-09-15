mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
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
pub use ray::Ray;
pub use sphere::Sphere;
