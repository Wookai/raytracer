use rand::Rng;
use std::ops;
use std::ops::AddAssign;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }
    pub fn zeros() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }
    pub fn ones() -> Vector {
        Vector::new(1.0, 1.0, 1.0)
    }
    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Vector {
        Vector::new(rng.gen(), rng.gen(), rng.gen())
    }
    pub fn random_in_range(rng: &mut rand::rngs::ThreadRng, min: f32, max: f32) -> Vector {
        Vector::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
    pub fn random_in_unit_sphere(rng: &mut rand::rngs::ThreadRng) -> Vector {
        loop {
            let v = Vector::random_in_range(rng, -1.0, 1.0);
            if v.norm_squared() < 1.0 {
                return v;
            }
        }
    }
    pub fn random_unit_vector(rng: &mut rand::rngs::ThreadRng) -> Vector {
        Vector::random_in_unit_sphere(rng).as_unit_vector()
    }
    pub fn random_in_hemisphere(rng: &mut rand::rngs::ThreadRng, normal: &Vector) -> Vector {
        let in_unit_sphere = Vector::random_in_unit_sphere(rng);
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            in_unit_sphere * -1.0
        }
    }
    pub fn norm_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }
    pub fn as_unit_vector(&self) -> Vector {
        self / self.norm()
    }
    pub fn dot(&self, rhs: &Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn is_almost_zero(&self) -> bool {
        let sensitivity = 1e-8;
        self.x.abs() < sensitivity && self.y.abs() < sensitivity && self.z.abs() < sensitivity
    }
    pub fn reflect(&self, normal: &Vector) -> Vector {
        self - 2.0 * self.dot(normal) * normal
    }
    pub fn refract(&self, normal: &Vector, etai_over_etat: f32) -> Vector {
        let cos_theta = (-1.0 * self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}

impl_op_ex!(*|a: &Vector, b: &Vector| -> Vector { Vector::new(a.x * b.x, a.y * b.y, a.z * b.z) });
impl_op_ex!(+|a: &Vector, b: &Vector| -> Vector { Vector::new(a.x + b.x, a.y + b.y, a.z + b.z)});
impl_op_ex!(-|a: &Vector, b: &Vector| -> Vector { Vector::new(a.x - b.x, a.y - b.y, a.z - b.z) });
impl_op_ex_commutative!(+|a: &Vector, b: f32| -> Vector {
    Vector::new(a.x + b, a.y + b, a.z + b)
});
impl_op_ex_commutative!(*|a: &Vector, b: f32| -> Vector { Vector::new(a.x * b, a.y * b, a.z * b) });
impl_op_ex_commutative!(/|a: &Vector, b: f32| -> Vector {
    Vector::new(a.x / b, a.y / b, a.z / b)
});

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
