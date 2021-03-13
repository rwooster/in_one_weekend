use super::color;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct PpmWriter {
    file: File,
}

impl PpmWriter {
    pub fn new(image_width: &usize, image_height: &usize) -> std::io::Result<PpmWriter> {
        let path = Path::new("image.ppm");

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file,
        };

        writeln!(file, "P3")?;
        writeln!(file, "{} {}", image_width, image_height)?;
        writeln!(file, "255")?;

        Ok(PpmWriter { file })
    }

    pub fn write_color(&mut self, pixel_color: color::Color) -> std::io::Result<()> {
        writeln!(
            self.file,
            "{} {} {}",
            pixel_color.0 as i32, pixel_color.1 as i32, pixel_color.2 as i32
        )?;
        Ok(())
    }
}
