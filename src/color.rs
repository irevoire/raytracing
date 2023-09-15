use crate::Color;

pub fn write_color(color: Color) {
    println!(
        "{} {} {}",
        (255.999 * color.r()) as usize,
        (255.999 * color.g()) as usize,
        (255.999 * color.b()) as usize
    );
}
