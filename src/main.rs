use std::sync::Arc;

use raytracing::*;

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Arc::new(material::Lambertian::new(Color::from(0.8, 0.8, 0.0)));
    let material_center = Arc::new(material::Dielectric::new(1.5));
    let material_left = Arc::new(material::Dielectric::new(1.5));
    let material_right = Arc::new(material::Metal::new(Color::from(0.8, 0.6, 0.2), 1.));

    #[rustfmt::skip]
    {
        world.add(Arc::new(Sphere::new(Point3::from( 0.0, -100.5, -1.0), 100.0, material_ground)));
        world.add(Arc::new(Sphere::new(Point3::from( 0.0,    0.0, -1.0),   0.5, material_center)));
        world.add(Arc::new(Sphere::new(Point3::from(-1.0,    0.0, -1.0),   0.5, material_left)));
        world.add(Arc::new(Sphere::new(Point3::from( 1.0,    0.0, -1.0),   0.5, material_right)));
    };

    let mut cam = Camera::new();

    cam.render(&world);
}
