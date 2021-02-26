use super::vec3;

pub use vec3::Vec3 as Color;

pub static WHITE: Color = Color(1.0, 1.0, 1.0);
pub static BLUE: Color = Color(0.5, 0.7, 1.0);

pub fn print_color(color: Color) {
    let translated = color * 255.999;

    println!(
        "{} {} {}",
        translated.0 as i32, translated.1 as i32, translated.2 as i32
    );
}
