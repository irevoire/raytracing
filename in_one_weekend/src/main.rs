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
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as usize;
            let ig = (255.999 * g) as usize;
            let ib = (255.999 * b) as usize;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone");
}
