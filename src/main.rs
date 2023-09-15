use raytracing::{write_color, Color};

fn main() {
    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprintln!("Scanline remaining: {}", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.;

            let color = Color::from(r, g, b);
            write_color(color);
        }
    }
    eprintln!("done");
}
