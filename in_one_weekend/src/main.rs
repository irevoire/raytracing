mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use ray::Ray;
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

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Vec3(0., 0., -1.), 0.5, r);
    if t > 0.0 {
        let N = (r.at(t) - Vec3(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Vec3(N.x() + 1., N.y() + 1., N.z() + 1.);
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

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3(0., 0., 0.);
    let horizontal = Vec3(viewport_width, 0., 0.);
    let vertical = Vec3(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - Vec3(0., 0., focal_length);

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);

            color::write_color(pixel_color);
        }
    }

    eprintln!("\nDone");
}
