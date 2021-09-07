use crate::{hit_record::HitRecord, hittable::Hittable, Ray, Vec3};

pub struct HittableList {
    objects: Vec<Hittable>,
}

impl HittableList {
    pub fn new(objects: Vec<Hittable>) -> HittableList {
        HittableList { objects }
    }

    pub fn add(&mut self, obj: Hittable) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        let mut hit = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if obj.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit = true;
                closest_so_far = temp_rec.t;
                rec.from(&temp_rec);
            }
        }

        hit
    }
}
