use std::ops;

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
}

impl_op_ex!(+|a: &Vector, b: &Vector| -> Vector { Vector::new(a.x + b.x, a.y + b.y, a.z + b.z)});
impl_op_ex!(-|a: &Vector, b: &Vector| -> Vector { Vector::new(a.x - b.x, a.y - b.y, a.z - b.z) });
impl_op_ex_commutative!(+|a: &Vector, b: f32| -> Vector {
    Vector::new(a.x + b, a.y + b, a.z + b)
});
impl_op_ex_commutative!(*|a: &Vector, b: f32| -> Vector { Vector::new(a.x * b, a.y * b, a.z * b) });
impl_op_ex_commutative!(/|a: &Vector, b: f32| -> Vector {
    Vector::new(a.x / b, a.y / b, a.z / b)
});
