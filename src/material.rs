use crate::{Colour, HitRecord, Ray, Vec3};

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Colour },
    Metal { albedo: Colour },
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Colour::default(),
        }
    }
}

pub fn scatter(r_in: &Ray, rec: &HitRecord) -> (Colour, Ray, bool) {
    match rec.material {
        Material::Metal { albedo } => {
            let reflected = Vec3::reflect(&r_in.dir.normalized(), &rec.normal);

            let scattered = Ray::new(rec.p, reflected);
            let should_scatter = Vec3::dot(&scattered.dir, &rec.normal) > 0.0;
            (albedo, scattered, should_scatter)
        }
        Material::Lambertian { albedo } => {
            let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

            // Catch degenerate scatter direction
            if scatter_direction.near_zero() {
                scatter_direction = rec.normal;
            }

            let scattered = Ray::new(rec.p, scatter_direction);
            (albedo, scattered, true)
        }
    }
}
