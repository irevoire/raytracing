use std::sync::Arc;

use rand::Rng;
use raytracing::*;

fn main() {
    // World
    let mut world = HittableList::new();

    let ground_material = Arc::new(material::Lambertian::new(Color::from(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::from(0, -1000, 0),
        1000,
        ground_material,
    )));

    let mut rng = rand::thread_rng();

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = random_float();
            let center = Point3::from(
                a as f64 + 0.9 * random_float(),
                0.2,
                b as f64 + 0.9 * random_float(),
            );

            if (center - Point3::from(4, 0.2, 0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(material::Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_within_interval(Interval::from(0.5, 1));
                    let fuzz = rng.gen_range(0.0..=0.5);
                    let sphere_material = Arc::new(material::Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(material::Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(material::Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::from(0, 1, 0), 1.0, material1)));

    let material2 = Arc::new(material::Lambertian::new(Color::from(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::from(-4, 1, 0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(material::Metal::new(Color::from(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::from(4, 1, 0), 1.0, material3)));

    let mut cam = Camera::new();

    // cam.image_width = 2048;
    // cam.defocus_angle = 10.;
    // cam.focus_dist = 3.4;

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;

    cam.vfov = 20.;
    cam.lookfrom = Point3::from(13, 2, 3);
    cam.lookat = Point3::from(0, 0, 0);
    cam.vup = Vec3::from(0, 1, 0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
