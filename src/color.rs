use crate::{Color, Interval};

pub fn write_color(color: Color, sample_per_pixel: usize) {
    // Divide the color by the number of samples.
    let scale = 1. / sample_per_pixel as f64;

    let r = color.r() * scale;
    let g = color.g() * scale;
    let b = color.b() * scale;

    // Write the translated [0,255] value of each color component.
    let intensity = Interval::from(0., 1. - f64::EPSILON);

    // eprintln!("{} {} {}", color.r(), color.g(), color.b());

    println!(
        "{} {} {}",
        (256. * intensity.clamp(r)) as usize,
        (256. * intensity.clamp(g)) as usize,
        (256. * intensity.clamp(b)) as usize
    );
}
