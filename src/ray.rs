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
    pub fn color(
        &self,
        world: &HittableList,
        rng: &mut rand::rngs::ThreadRng,
        depth: i16,
    ) -> Color {
        // After too many bounces, no more light is gathered.
        if depth <= 0 {
            return Color::zeros();
        }

        if let Some(impact) = world.hit(&self, 0.0, f32::MAX) {
            let target = impact.point + impact.normal + Vector::random_in_unit_sphere(rng);
            let reflection = Ray {
                origin: impact.point,
                direction: target - impact.point,
            };
            return 0.5 * reflection.color(world, rng, depth - 1);
        }
        let unit_direction = self.direction.as_unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::ones() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
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
