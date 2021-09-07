use crate::{Ray, Vec3};

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
        self.normal = rec.normal.clone();
        self.t = rec.t;
        self.front_face = rec.front_face;
    }
}
