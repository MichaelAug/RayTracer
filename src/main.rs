use ray_tracer::material::*;
use ray_tracer::utils::*;
use ray_tracer::{Camera, Colour, Hittable, HittableList, Point3, Ray, Sphere};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn main() {
    //World
    let material_ground = Material::new_lambertian(Colour::new(0.8, 0.8, 0.0));
    let material_center = Material::new_lambertian(Colour::new(0.1, 0.2, 0.5));
    let material_left = Material::new_dielectric(1.5);
    let material_right = Material::new_metal(Colour::new(0.8, 0.6, 0.2), 1.0);

    let mut world = HittableList::default();
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground,
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: material_center,
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_left,
    }));

    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: material_left,
    }));

    world.add(Box::new(Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_right,
    }));

    //Camera
    let cam = Camera::default();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scan lines remaining: {}", j); //eprintln! flushes buffer
        for i in 0..IMAGE_WIDTH {
            let mut pixel_colour = Colour::default();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f64) + random_f64()) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_f64()) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, &world, MAX_DEPTH);
            }
            write_colour(&pixel_colour, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done");
}

fn write_colour(pixel_col: &Colour, samples_per_pixel: i32) {
    let mut r = pixel_col.x;
    let mut g = pixel_col.y;
    let mut b = pixel_col.z;

    // Divide colour by number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / (samples_per_pixel as f64);
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    println!(
        "{} {} {}",
        ((256.0 * f64::clamp(r, 0.0, 0.999)) as i32),
        ((256.0 * f64::clamp(g, 0.0, 0.999)) as i32),
        ((256.0 * f64::clamp(b, 0.0, 0.999)) as i32)
    )
}

fn ray_colour(r: &Ray, world: &impl Hittable, depth: i32) -> Colour {
    // If we exceed ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Colour::default();
    }

    // if hit object in world
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let (attenuation, scattered, should_scatter) = scatter(r, &rec);
        return if should_scatter {
            attenuation * ray_colour(&scattered, world, depth - 1)
        } else {
            Colour::default()
        };
    }

    // if did not hit anything
    let unit_dir = r.dir.normalized();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}
