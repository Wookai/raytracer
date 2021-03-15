use std::io::prelude::*;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn write_color(&self, mut file: &std::fs::File) -> std::io::Result<()> {
        writeln!(
            &mut file,
            "{:.0} {:.0} {:.0}",
            self.x * 255.0,
            self.y * 255.0,
            self.z * 255.0
        )?;
        Ok(())
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

impl_op_ex!(+ |a: &Vector, b: &Vector| -> Vector {
    Vector {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});
impl_op_ex!(-|a: &Vector, b: &Vector| -> Vector {
    Vector {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});
impl_op_ex_commutative!(+|a: &Vector, b: f32| -> Vector {
    Vector {
        x: a.x + b,
        y: a.y + b,
        z: a.z + b,
    }
});
impl_op_ex_commutative!(*|a: &Vector, b: f32| -> Vector {
    Vector {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});
impl_op_ex_commutative!(/|a: &Vector, b: f32| -> Vector { 
    Vector {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    } });
