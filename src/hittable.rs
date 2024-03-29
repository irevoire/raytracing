use std::sync::Arc;

use crate::{material, Color, Interval, Material, Point3, Ray, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Default::default(),
            normal: Default::default(),
            material: Arc::new(material::Lambertian::new(Color::default())),
            t: Default::default(),
            front_face: Default::default(),
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
