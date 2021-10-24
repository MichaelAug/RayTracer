use crate::{Colour, HitRecord, Ray, Vec3};

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Colour },
    Metal { albedo: Colour, fuzz: f64 },
}

impl Material {
    pub fn new_lambertian(albedo: Colour) -> Self {
        Self::Lambertian { albedo }
    }
    pub fn new_metal(albedo: Colour, fuzz: f64) -> Self {
        Self::Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
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
        Material::Metal { albedo, fuzz } => {
            let reflected = Vec3::reflect(&r_in.dir.normalized(), &rec.normal);

            let scattered = Ray::new(rec.p, reflected + fuzz * Vec3::random_in_unit_sphere());
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
