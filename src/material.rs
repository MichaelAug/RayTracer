use crate::utils::random_f64;
use crate::{Colour, HitRecord, Ray, Vec3};

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Colour },
    Metal { albedo: Colour, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
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

    pub fn new_dielectric(index_of_refraction: f64) -> Self {
        Self::Dielectric {
            index_of_refraction,
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
            let reflected = Vec3::reflect(r_in.dir.normalized(), rec.normal);

            let scattered = Ray::new(rec.p, reflected + fuzz * Vec3::random_in_unit_sphere());
            let should_scatter = Vec3::dot(scattered.dir, rec.normal) > 0.0;
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
        Material::Dielectric {
            index_of_refraction,
        } => {
            let refraction_ratio = if rec.front_face {
                1.0 / index_of_refraction
            } else {
                index_of_refraction
            };

            let unit_dir = r_in.dir.normalized();
            let cos_theta = f64::min(Vec3::dot(-unit_dir, rec.normal), 1.0);
            let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

            let cannot_refract = refraction_ratio * sin_theta > 1.0;

            let direction =
                if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_f64() {
                    Vec3::reflect(unit_dir, rec.normal)
                } else {
                    Vec3::refract(unit_dir, rec.normal, refraction_ratio)
                };

            let scattered = Ray::new(rec.p, direction);
            (Colour::new(1.0, 1.0, 1.0), scattered, true)
        }
    }
}
// Schlick's approximation for reflectance
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
}
