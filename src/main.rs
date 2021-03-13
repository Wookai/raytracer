use indicatif::ProgressBar;
use std::fs::File;
use std::io::prelude::*;
#[macro_use]
extern crate impl_ops;
use std::ops;

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    fn write_color(&self, mut file: &std::fs::File) -> std::io::Result<()> {
        writeln!(
            &mut file,
            "{:.0} {:.0} {:.0}",
            self.x * 255.0,
            self.y * 255.0,
            self.z * 255.0
        )?;
        Ok(())
    }

    fn norm_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }

    fn as_unit_vector(&self) -> Vector {
        self / self.norm()
    }

    fn dot(&self, rhs: &Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl_op_ex!(+ |a: &Vector, b: &Vector| -> Vector {
    Vector {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});
impl_op_ex!(-|a: &Vector, b: &Vector| -> Vector {
    Vector {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});
impl_op_ex_commutative!(+|a: &Vector, b: f32| -> Vector {
    Vector {
        x: a.x + b,
        y: a.y + b,
        z: a.z + b,
    }
});
impl_op_ex_commutative!(*|a: &Vector, b: f32| -> Vector {
    Vector {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});
impl_op_ex_commutative!(/|a: &Vector, b: f32| -> Vector { 
    Vector {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    } });

use Vector as Point;
use Vector as Color;

#[derive(Debug)]
struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    fn at(&self, time: f32) -> Point {
        self.origin + self.direction * time
    }
    fn color(&self, sphere: &Sphere) -> Color {
        let t = sphere.is_hit_by(&self);
        if t > 0.0 {
            let normal = (self.at(t) - sphere.center).as_unit_vector();
            return 0.5 * (normal + 1.0);
        }
        let unit_direction = self.direction.as_unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        } * (1.0 - t)
            + Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            } * t
    }
}

#[derive(Debug)]
struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    fn is_hit_by(&self, ray: &Ray) -> f32 {
        let oc: Vector = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = f32::powi(half_b, 2) - a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}

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

    let mut file = File::create("foo.ppm")?;

    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;

    let bar = ProgressBar::new(image_height.into());

    let sphere = Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    };

    for y in (0..image_height).rev() {
        bar.inc(1);
        for x in 0..image_width {
            let u = x as f32 / (image_width as f32 - 1.0);
            let v = y as f32 / (image_height as f32 - 1.0);
            let ray = Ray {
                origin,
                direction: lower_left_corner + horizontal * u + vertical * v - origin,
            };
            ray.color(&sphere).write_color(&file)?;
        }
    }

    Ok(())
}
