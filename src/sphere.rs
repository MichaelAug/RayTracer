use crate::material::Material;
use crate::Vec3;

use super::{HitRecord, Hittable, Point3};

#[derive(Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Material) -> Self {
        Sphere {
            center: cen,
            radius: r,
            material: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec = HitRecord::default();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.material = self.material;

        Some(rec)
    }
}
