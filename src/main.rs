use raytracing::{write_color, Color, Point3, Ray, Vec3};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f64 / aspect_ratio).ceil() as usize;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::from(viewport_width, 0, 0);
    let viewport_v = Vec3::from(0, -viewport_height, 0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::from(0, 0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprintln!("Scanline remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let color = ray_color(&r);
            write_color(color);
        }
    }
    eprintln!("done");
}

pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;

    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;

    let discriminant = b * b - 4. * a * c;
    discriminant >= 0.
}

pub fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::from(0, 0, -1), 0.5, r) {
        return Color::from(1, 0, 0);
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * unit_direction.y() + 1.0;
    (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
}
