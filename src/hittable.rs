use super::{Point3, Vec3, Ray};

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal} else {-*outward_normal};
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
    
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if let Some(temp_record) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                rec = Some(temp_record);
            }
        }

        return rec;
    }
}