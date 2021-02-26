mod vec3;


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

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("");
}

fn main() {

    let v = vec3::Vec3(1.0, 2.0, 3.0);
    let v2 = vec3::Vec3(1.0, 2.0, 3.0);

    println!("{}", v);
    println!("{:?}", v);

    println!("{}", v + v2);
}
