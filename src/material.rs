use crate::ray::{Ray, RayImpact};
use crate::vector::Vector;
use rand::rngs::ThreadRng;

use Vector as Color;

pub trait Material {
    fn scatter(&self, ray: &Ray, impact: &RayImpact, rng: &mut ThreadRng) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}
impl Material for Lambertian {
    fn scatter(&self, _: &Ray, impact: &RayImpact, rng: &mut ThreadRng) -> Option<(Ray, Color)> {
        let mut scatter_direction = impact.normal + Vector::random_unit_vector(rng);

        // Catch degenerate scatter direction
        if scatter_direction.is_almost_zero() {
            scatter_direction = impact.normal;
        }

        let scattered_ray = Ray {
            origin: impact.point,
            direction: scatter_direction,
        };
        Some((scattered_ray, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32, // Fuzzing factor for reflections, similar to brushed metal. 0 means no fuzzing.
}
impl Material for Metal {
    fn scatter(&self, ray: &Ray, impact: &RayImpact, rng: &mut ThreadRng) -> Option<(Ray, Color)> {
        let reflection = ray.direction.as_unit_vector().reflect(&impact.normal)
            + self.fuzz.clamp(0.0, 1.0) * Vector::random_in_unit_sphere(rng);

        if reflection.dot(&impact.normal) > 0.0 {
            let scattered_ray = Ray {
                origin: impact.point,
                direction: reflection,
            };
            Some((scattered_ray, self.albedo))
        } else {
            None
        }
    }
}
