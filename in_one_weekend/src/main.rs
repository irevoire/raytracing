mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod rand;
mod ray;
mod sphere;
mod vec3;

use crate::rand::random_double;
use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
pub use vec3::{Color, Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.orig - center;
    let a = r.dir.length_squared();
    let half_b = oc.dot(&r.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0. {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray, world: &impl Hittable, depth: usize) -> Color {
    if depth <= 0 {
        return Vec3(0., 0., 0.);
    }

    let mut rec = HitRecord::default();

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal.random_in_hemisphere();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::default();
    world.add(Sphere::new(Vec3(0., 0., -1.), 0.5));
    world.add(Sphere::new(Vec3(0., -100.5, -1.), 100.));

    // Camera
    let cam = Camera::new();

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            color::write_color(pixel_color, samples_per_pixel);
        }
    }

    eprintln!("\nDone");
}
