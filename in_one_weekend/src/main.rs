mod color;
mod vec3;

pub use vec3::{Color, Vec3};

fn main() {
    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color = Vec3(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            color::write_color(pixel_color);
        }
    }

    eprintln!("\nDone");
}
