use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::{Ray, RayImpact};
use crate::vector::Vector;
use std::rc::Rc;

use Vector as Point;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Rc<dyn Material>,
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
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        Some(RayImpact::new(
            &point,
            root,
            ray,
            &outward_normal,
            Rc::clone(&self.material),
        ))
    }
}
