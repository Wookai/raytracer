use indicatif::ProgressBar;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate impl_ops;

mod camera;
mod hittable;
mod ray;
mod sphere;
mod vector;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::sphere::*;
use crate::vector::*;

use Vector as Color;
use Vector as Point;

fn write_color(color: &Color, mut file: &std::fs::File) -> std::io::Result<()> {
    writeln!(
        &mut file,
        "{:.0} {:.0} {:.0}",
        color.x * 255.0,
        color.y * 255.0,
        color.z * 255.0
    )?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let camera = Camera::new(aspect_ratio);

    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.objects.push(Box::new(Sphere {
        center: Point::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.objects.push(Box::new(Sphere {
        center: Point::new(0.0, -100.5, -1.0),
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
            let color = camera.get_ray(u, v).color(&world);
            write_color(&color, &file)?;
        }
    }

    Ok(())
}
