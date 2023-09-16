use std::sync::Arc;

use raytracing::*;

fn main() {
    // World
    let mut world = HittableList::new();

    world.add(Arc::new(Sphere::new(
        Point3::from(0, 0, -1),
        0.5,
        Arc::new(material::Lambertian::new(Color::new())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::from(0, -100.5, -1),
        100,
        Arc::new(material::Lambertian::new(Color::new())),
    )));

    let mut cam = Camera::new();

    cam.render(&world);
}
