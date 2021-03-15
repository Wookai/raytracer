use indicatif::ProgressBar;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate impl_ops;

mod hittable;
mod ray;
mod sphere;
mod vector;

use crate::hittable::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vector::*;

use Vector as Point;

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vector {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vector {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner: Vector = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vector {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.objects.push(Box::new(Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    }));
    world.objects.push(Box::new(Sphere {
        center: Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    }));

    let mut file = File::create("foo.ppm")?;
    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;

    let bar = ProgressBar::new(image_height.into());
    for y in (0..image_height).rev() {
        bar.inc(1);
        for x in 0..image_width {
            let u = x as f32 / (image_width as f32 - 1.0);
            let v = y as f32 / (image_height as f32 - 1.0);
            let ray = Ray {
                origin,
                direction: lower_left_corner + horizontal * u + vertical * v - origin,
            };
            ray.color(&world).write_color(&file)?;
        }
    }

    Ok(())
}
