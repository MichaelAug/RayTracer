use rand::{thread_rng, Rng};
use ray_tracer::{Camera, Colour, Hittable, HittableList, Point3, Ray, Sphere};
use std::f64::consts::PI;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;

fn main() {
    //World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    //Camera
    let cam = Camera::new();

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
                pixel_colour += ray_colour(&r, &world);
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

    let scale = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    println!(
        "{} {} {}",
        ((256.0 * f64::clamp(r, 0.0, 0.999)) as i32),
        ((256.0 * f64::clamp(g, 0.0, 0.999)) as i32),
        ((256.0 * f64::clamp(b, 0.0, 0.999)) as i32)
    )
}

fn ray_colour(r: &Ray, world: &impl Hittable) -> Colour {
    // if hit object in world
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0));
    }

    // if did not hit anything
    let unit_dir = r.dir.normalized();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// random number in range [0, 1)
fn random_f64() -> f64 {
    thread_rng().gen()
}

// random number in range [min, max]
fn random_f64_in_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..=max)
}
