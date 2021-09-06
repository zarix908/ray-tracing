use std::{fs::File, io, io::prelude::*};
mod vec3;
use vec3::Vec3;
mod ray;
use ray::Ray;

fn ray_color(ray: &Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        let t = 0.5 * (ray.direction().normalize().y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn write_color(file: &mut File, color: &Vec3) -> Result<(), io::Error> {
    writeln!(
        file,
        "{} {} {}",
        (color.x() * 255.999) as u8,
        (color.y() * 255.999) as u8,
        (color.z() * 255.999) as u8,
    )
}

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - &center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let disc = b * b - 4.0 * a * c;
    disc > 0.0
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as i32;

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // render

    let mut img = File::create("output/image.ppm").unwrap();

    writeln!(img, "P3\n{} {}\n255", img_width, img_height).unwrap();

    for j in (0..img_height).rev() {
        println!("scanlines remaining: {}", j);
        io::stdout().flush().unwrap();

        for i in 0..img_width {
            let u = i as f64 / (img_width as f64 - 1.0);
            let v = j as f64 / (img_height as f64 - 1.0);

            let ray = Ray::new(
                origin.clone(),
                &(&lower_left_corner + &(u * &horizontal) + v * &vertical) - &origin,
            );

            write_color(&mut img, &ray_color(&ray)).unwrap();
        }
    }
}
