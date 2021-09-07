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
mod camera;
use camera::Camera;
use rand::Rng;

fn ray_color(ray: &Ray, world: &Hittable) -> Vec3 {
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

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn write_color(file: &mut File, color: &Vec3, samples_per_pixel: i32) -> Result<(), io::Error> {
    let ratio = 1.0 / samples_per_pixel as f64;

    writeln!(
        file,
        "{} {} {}",
        (256.0 * clamp(color.x() * ratio, 0.0, 0.999)) as u8,
        (256.0 * clamp(color.y() * ratio, 0.0, 0.999)) as u8,
        (256.0 * clamp(color.z() * ratio, 0.0, 0.999)) as u8,
    )
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // camera
    let c = Camera::new();

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
            let mut color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let mut rng = rand::thread_rng();
                let u = (i as f64 + rng.gen::<f64>()) / (img_width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (img_height as f64 - 1.0);

                let ray = c.get_ray(u, v);
                color += &ray_color(&ray, &world)
            }

            write_color(&mut img, &color, samples_per_pixel).unwrap();
        }
    }
}
