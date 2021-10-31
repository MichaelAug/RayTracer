use ray_tracer::material::*;
use ray_tracer::utils::*;
use ray_tracer::{Camera, Colour, Hittable, HittableList, Point3, Ray, Sphere, Vec3};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_scene();

    //Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

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

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Material::new_lambertian(Colour::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                (a as f64) + 0.9 * random_f64(),
                0.2,
                (b as f64) + 0.9 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Colour::random() * Colour::random();
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::new_lambertian(albedo),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Colour::random_in_range(0.5, 1.0);
                    let fuzz = random_f64_in_range(0.0, 0.5);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::new_metal(albedo, fuzz),
                    )));
                } else {
                    // glass
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::new_dielectric(1.5),
                    )));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Material::new_dielectric(1.5),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::new_lambertian(Colour::new(0.4, 0.2, 0.1)),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Material::new_metal(Colour::new(0.7, 0.6, 0.5), 0.0),
    )));

    world
}
