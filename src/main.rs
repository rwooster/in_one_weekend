mod vec3;
mod color;
mod ray;


fn make_image() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);

        for i in 0..image_width {
            let r = (i as f32) / (image_width - 1) as f32;
            let g = (j as f32) / (image_height - 1) as f32;
            let b = 0.25;

            let pixel_color = color::Color(r, g, b);
            color::print_color(pixel_color);
        }
    }
    eprintln!("");
}

fn main() {
    make_image();
}
