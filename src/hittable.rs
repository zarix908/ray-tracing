use crate::{hittable_list::HittableList, HitRecord, Ray, Sphere};

pub enum Hittable {
    List(HittableList),
    Sphere(Sphere),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        match &self {
            Hittable::List(l) => l.hit(ray, t_min, t_max, rec),
            Hittable::Sphere(s) => s.hit(ray, t_min, t_max, rec),
        }
    }
}
