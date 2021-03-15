use crate::ray::{Ray, RayImpact};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayImpact>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
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
