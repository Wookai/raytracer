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
    fn color(&self, world: &HittableList) -> Color {
        match world.hit(&self, 0.0, f32::MAX) {
            Some(impact) => {
                return 0.5
                    * (impact.normal
                        + Color {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        })
            }
            None => (),
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

struct RayImpact {
    point: Point,
    normal: Vector,
    t: f32,
    front_face: bool, // is the ray impact from the outside?
}

impl RayImpact {
    fn new(point: &Vector, t: f32, ray: &Ray, outward_normal: &Vector) -> RayImpact {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            outward_normal * -1.0
        };
        RayImpact {
            point: *point,
            normal,
            t,
            front_face,
        }
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayImpact>;
}

#[derive(Debug)]
struct Sphere {
    center: Point,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayImpact> {
        let oc: Vector = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = f32::powi(half_b, 2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range of t
        let root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || root > t_max {
            let root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        Some(RayImpact::new(&point, root, ray, &outward_normal))
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayImpact> {
        let mut closest_impact: Option<RayImpact> = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(impact) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = impact.t;
                closest_impact = Some(impact);
            }
        }
        closest_impact
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
