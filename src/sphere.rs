use crate::{HitRecord, Ray, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().sqr_len();
        let half_b = oc.dot(ray.direction());
        let c = oc.sqr_len() - self.radius * self.radius;

        let disc = half_b * half_b - a * c;

        if disc < 0.0 {
            return false;
        }

        let mut root = (-half_b - disc.sqrt()) / a;
        if root < t_min || root > t_max {
            root = (-half_b + disc.sqrt()) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (&rec.p - &self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}
