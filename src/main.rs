use std::sync::Arc;

use raytracing::*;

fn main() {
    // World
    let mut world = HittableList::new();

    world.add(Arc::new(Sphere::new(Point3::from(0, 0, -1), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::from(0, -100.5, -1), 100)));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    cam.render(&world);
}
