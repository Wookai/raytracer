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
}
impl Material for Metal {
    fn scatter(&self, ray: &Ray, impact: &RayImpact, _: &mut ThreadRng) -> Option<(Ray, Color)> {
        let reflection = ray.direction.as_unit_vector().reflect(&impact.normal);

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
