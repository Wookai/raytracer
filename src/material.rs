use crate::ray::{Ray, RayImpact};
use crate::vector::Vector;
use rand::rngs::ThreadRng;
use rand::Rng;

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

pub struct Dielectric {
    pub index_of_refraction: f32,
}
impl Dielectric {
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, impact: &RayImpact, rng: &mut ThreadRng) -> Option<(Ray, Color)> {
        let attenuation = Color::ones();
        let refraction_ratio = if impact.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = ray.direction.as_unit_vector();
        let cos_theta = (-1.0 * unit_direction).dot(&impact.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen() {
                unit_direction.reflect(&impact.normal)
            } else {
                unit_direction.refract(&impact.normal, refraction_ratio)
            };
        Some((
            Ray {
                origin: impact.point,
                direction,
            },
            attenuation,
        ))
    }
}
