use indicatif::ProgressBar;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

#[macro_use]
extern crate impl_ops;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vector;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::*;
use crate::sphere::*;
use crate::vector::*;

use Vector as Color;
use Vector as Point;

fn write_color(
    color_stack: &Color,
    samples_per_pixel: u32,
    mut file: &std::fs::File,
) -> std::io::Result<()> {
    let r = color_stack.x;
    let g = color_stack.y;
    let b = color_stack.z;

    // Rescale colors by the number of samples and gamma-correct with gamma=2.0
    let scale = 1.0 / samples_per_pixel as f32;
    let rescale =
        |v: f32| -> i32 { ((v * scale).sqrt().clamp(0.0, 0.9999) * 256.0).trunc() as i32 };

    writeln!(&mut file, "{} {} {}", rescale(r), rescale(g), rescale(b))?;
    Ok(())
}

fn random_scene(rng: &mut rand::rngs::ThreadRng) -> HittableList {
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.objects.push(Box::new(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Rc::clone(&ground_material),
    }));

    let fraction_lambertian = 0.8;
    let fraction_metal = 0.15;
    let reference_point = Point::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let center = Point::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - reference_point).norm() > 0.9 {
                let choose_material: f32 = rng.gen();
                let material: Rc<dyn Material>;
                if choose_material < fraction_lambertian {
                    material = Rc::new(Lambertian {
                        albedo: Color::random(rng) * Color::random(rng),
                    });
                } else if choose_material < (fraction_lambertian + fraction_metal) {
                    material = Rc::new(Metal {
                        albedo: Color::random_in_range(rng, 0.5, 1.0),
                        fuzz: rng.gen_range(0.0..0.5),
                    });
                } else {
                    material = Rc::new(Dielectric {
                        index_of_refraction: 1.5,
                    });
                }

                world.objects.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material,
                }));
            }
        }
    }

    world.objects.push(Box::new(Sphere {
        center: Point::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Dielectric {
            index_of_refraction: 1.5,
        }),
    }));
    world.objects.push(Box::new(Sphere {
        center: Point::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    }));
    world.objects.push(Box::new(Sphere {
        center: Point::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    world
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_ray_depth: i16 = 50;

    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::zeros();
    let up_direction = Point::new(0.0, 1.0, 0.0);
    let vertical_field_of_view_degrees = 20.0;
    let distance_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up_direction,
        vertical_field_of_view_degrees,
        aspect_ratio,
        aperture,
        distance_to_focus,
    );

    let mut rng = rand::thread_rng();
    let world = random_scene(&mut rng);

    let mut file = File::create("foo.ppm")?;
    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;

    let bar = ProgressBar::new(image_height.into());
    for y in (0..image_height).rev() {
        bar.inc(1);
        for x in 0..image_width {
            let mut color = Color::zeros();
            for _ in 0..samples_per_pixel {
                let u = (x as f32 + rng.gen::<f32>()) / (image_width as f32 - 1.0);
                let v = (y as f32 + rng.gen::<f32>()) / (image_height as f32 - 1.0);
                color += camera
                    .get_ray(u, v, &mut rng)
                    .color(&world, &mut rng, max_ray_depth);
            }
            write_color(&color, samples_per_pixel, &file)?;
        }
    }

    Ok(())
}
