use crate::{Vec3, Ray};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { 1.0 } else { -1.0 } * outward_normal;
    }

    pub fn from(&mut self, rec: &HitRecord) {
        self.p = rec.p.clone();
        self.normal = rec.p.clone();
        self.t = rec.t;
        self.front_face = rec.front_face;
    }
}

pub trait Hittable {
    fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}