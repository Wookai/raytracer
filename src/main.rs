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

fn create_world() -> HittableList {
    // Prepare materials
    let ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let center = Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    };
    let metal_left = Metal {
        albedo: Color::ones() * 0.8,
    };
    let metal_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
    };

    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.objects.push(Box::new(Sphere {
        center: Point::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(ground),
    }));
    world.objects.push(Box::new(Sphere {
        center: Point::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(center),
    }));
    world.objects.push(Box::new(Sphere {
        center: Point::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(metal_left),
    }));
    world.objects.push(Box::new(Sphere {
        center: Point::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(metal_right),
    }));

    world
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_ray_depth: i16 = 50;

    let camera = Camera::new(aspect_ratio);
    let world = create_world();

    let mut file = File::create("foo.ppm")?;
    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;

    let mut rng = rand::thread_rng();

    let bar = ProgressBar::new(image_height.into());
    for y in (0..image_height).rev() {
        bar.inc(1);
        for x in 0..image_width {
            let mut color = Color::zeros();
            for _ in 0..samples_per_pixel {
                let u = (x as f32 + rng.gen::<f32>()) / (image_width as f32 - 1.0);
                let v = (y as f32 + rng.gen::<f32>()) / (image_height as f32 - 1.0);
                color += camera.get_ray(u, v).color(&world, &mut rng, max_ray_depth);
            }
            write_color(&color, samples_per_pixel, &file)?;
        }
    }

    Ok(())
}
