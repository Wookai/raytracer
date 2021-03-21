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
    pub fn new(
        look_from: Point,
        look_at: Point,
        up_direction: Vector,
        vertical_field_of_view_degrees: f32,
        aspect_ratio: f32,
    ) -> Camera {
        let theta = vertical_field_of_view_degrees * std::f32::consts::PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let reverse_direction_of_view = (look_from - look_at).as_unit_vector();
        let image_right_direction = up_direction.cross(&reverse_direction_of_view);
        let iamge_up_direction = reverse_direction_of_view.cross(&image_right_direction);

        let origin = look_from;
        let horizontal = viewport_width * image_right_direction;
        let vertical = viewport_height * iamge_up_direction;
        let lower_left_corner: Vector =
            origin - horizontal / 2.0 - vertical / 2.0 - reverse_direction_of_view;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin,
        }
    }
}
