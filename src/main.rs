use std::{fs::File, io, io::prelude::*};
mod vec3;
use vec3::Vec3;
mod ray;
use ray::Ray;
mod hit_record;
mod hittable;
mod hittable_list;
mod sphere;
use hit_record::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;

fn ray_color(ray: &Ray, world: &Hittable) -> Vec3 {
    // let s = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let mut rec = HitRecord {
        p: Vec3::new(0.0, 0.0, 0.0),
        t: 0.0,
        normal: Vec3::new(0.0, 0.0, 0.0),
        front_face: false,
    };

    if world.hit(ray, 0.0, f64::INFINITY, &mut rec) {
        0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0))
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

    // world
    let world = Hittable::List(HittableList::new(vec![
        Hittable::Sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Hittable::Sphere(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]));

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

            write_color(&mut img, &ray_color(&ray, &world)).unwrap();
        }
    }
}
