use std::sync::Arc;

use raytracing::*;

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Arc::new(material::Lambertian::new(Color::from(0.8, 0.8, 0.0)));
    let material_center = Arc::new(material::Lambertian::new(Color::from(0.1, 0.2, 0.5)));
    let material_left = Arc::new(material::Dielectric::new(1.5));
    let material_right = Arc::new(material::Metal::new(Color::from(0.8, 0.6, 0.2), 0.));

    #[rustfmt::skip]
    {
        world.add(Arc::new(Sphere::new(Point3::from( 0.0, -1000.5, -1.0), 1000.0, material_ground)));
        world.add(Arc::new(Sphere::new(Point3::from( 0.0,    0.0, -1.0),   0.5, material_center)));
        world.add(Arc::new(Sphere::new(Point3::from(-1.0,    0.0, -1.0),   -0.4, material_left)));
        world.add(Arc::new(Sphere::new(Point3::from( 1.0,    0.0, -1.0),   0.5, material_right)));
    };

    let mut cam = Camera::new();

    cam.vfov = 20.;
    cam.samples_per_pixel = 100;
    cam.image_width = 2048;
    cam.defocus_angle = 10.;
    cam.focus_dist = 3.4;

    cam.lookfrom = Point3::from(-2, 2, 1);
    cam.lookat = Point3::from(0, 0, -1);
    cam.vup = Vec3::from(0, 1, 0);

    cam.render(&world);
}
