use crate::ray::Ray;
use crate::vector::Vector;

use Vector as Point;

pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Vector,
    pub vertical: Vector,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Camera {
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

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin,
        }
    }
}
