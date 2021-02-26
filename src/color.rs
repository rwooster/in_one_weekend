use super::vec3;

pub use vec3::Vec3 as Color;

pub fn print_color(color: Color) {
    let translated = color * 255.999;

    println!("{} {} {}", translated.0 as i32, translated.1 as i32, translated.2 as i32);
}
