use crate::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: usize) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divid the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1. / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    println!(
        "{} {} {}",
        (256. * r.clamp(0.0, 0.999)) as usize,
        (256. * g.clamp(0.0, 0.999)) as usize,
        (256. * b.clamp(0.0, 0.999)) as usize,
    );
}
