use crate::ray::Ray;
use crate::vector::Vector;

use Vector as Point;

pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub horizontal_direction: Vector,
    pub vertical_direction: Vector,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        up_direction: Vector,
        vertical_field_of_view_degrees: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Camera {
        let theta = vertical_field_of_view_degrees * std::f32::consts::PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let reverse_direction_of_view = (look_from - look_at).as_unit_vector();
        let image_right_direction = up_direction
            .cross(&reverse_direction_of_view)
            .as_unit_vector();
        let image_up_direction = reverse_direction_of_view.cross(&image_right_direction);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * image_right_direction;
        let vertical = focus_distance * viewport_height * image_up_direction;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * reverse_direction_of_view;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            horizontal_direction: image_right_direction,
            vertical_direction: image_up_direction,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut rand::rngs::ThreadRng) -> Ray {
        let random_displacement = self.lens_radius * Vector::random_in_unit_disk(rng);
        let offset = self.horizontal_direction * random_displacement.x
            + self.vertical * random_displacement.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
        }
    }
}
