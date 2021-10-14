use ray_tracer::{Colour, HitRecord, Hittable, HittableList, Point3, Ray, Sphere, Vec3};
use std::f64::consts::PI;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
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
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scan lines remaining: {}", j); //eprintln! flushes buffer
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let pixel_col = ray_colour(&r, &world);

            write_colour(&pixel_col);
        }
    }
    eprintln!("Done");
}

fn write_colour(pixel_col: &Colour) {
    println!(
        "{} {} {}",
        ((pixel_col.x * 255.999) as i32),
        ((pixel_col.y * 255.999) as i32),
        ((pixel_col.z * 255.999) as i32)
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
