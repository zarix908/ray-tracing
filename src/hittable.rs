use crate::{Vec3, Ray};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}