use crate::hittable::{Hittable, HittableList};
use crate::vector::Vector;

use Vector as Point;
use Vector as Color;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn at(&self, time: f32) -> Point {
        self.origin + self.direction * time
    }
    pub fn color(&self, world: &HittableList) -> Color {
        if let Some(impact) = world.hit(&self, 0.0, f32::MAX) {
            return 0.5
                * (impact.normal
                    + Color {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    });
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

pub struct RayImpact {
    pub point: Point,
    pub normal: Vector,
    pub t: f32,
    pub front_face: bool, // is the ray impact from the outside?
}

impl RayImpact {
    pub fn new(point: &Vector, t: f32, ray: &Ray, outward_normal: &Vector) -> RayImpact {
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
