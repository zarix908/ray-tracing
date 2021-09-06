use crate::hittable::Hittable;

pub struct HittableList {
    objects: Vec<dyn Hittable>;
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec<dyn Hittable>::new() }
    }

    pub fn from(obj dyn Hittable) -> HittableList {
        HittableList { objects: vec![obj] }
    }

    pub fn add(&mut self, obj dyn Hittable) {
        self.objects.push(obj);
    } 

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let temp_rec = HitRecord{
            p: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        let hit = false;
        let closest_so_far = t_max;

        for obj in self.objects {
            if obj.hit(ray, t_min, closest_so_far, temp_rec) {
                hit = true;
                closest_so_far = temp_rec.t;
                rec.from(&temp_rec);
            }
        }

        hit
    }
}
