use std::{fs::File, io, io::prelude::*};
mod vec3;
use vec3::Vec3;

fn main() {
    println!(
        "{:?}",
        Vec3::new(1.0, 2.0, 3.0).cross(&Vec3::new(1.0, 5.0, 7.0))
    );

    // let img_width = 256;
    // let img_height = 256;

    // let mut img = File::create("output/image.ppm").unwrap();

    // writeln!(img, "P3\n{} {}\n255", img_width, img_height).unwrap();

    // for j in (0..img_height).rev() {
    //     println!("scanlines remaining: {}", j);
    //     io::stdout().flush().unwrap();
    //     for i in 0..img_width {
    //         let r = i as f64 / (img_width - 1) as f64 * 255.999;
    //         let g = j as f64 / (img_height - 1) as f64 * 255.999;
    //         let b = 0.25 * 255.999;

    //         writeln!(img, "{} {} {}", r as u8, g as u8, b as u8).unwrap();
    //     }
    // }
}
