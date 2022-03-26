use std::fmt;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec4(pub [f32; 4]);

impl fmt::Display for Vec4 {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})\n", self[0], self[1], self[2], self[3])
    }
}

impl Vec4 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Vec4([0.; 4])
    }
    #[allow(dead_code)]
    pub fn from_data(data: &[f32]) -> Self {
        let mut v = Self::new();
        v[0] = data[0];
        v[1] = data[1];
        v[2] = data[2];
        v[3] = data[3];
        v
    }
    #[allow(dead_code)]
    pub fn data<'a>(&'a self) -> &'a [f32] {
        &self.0[..]
    }
    #[allow(dead_code)]
    pub fn mul(&self, k: f32) -> Self {
        Vec4([self[0] * k, self[1] * k, self[2] * k, self[3] * k])
    }
    #[allow(dead_code)]
    pub fn div(&self, k: f32) -> Self {
        Vec4([self[0] / k, self[1] / k, self[2] / k, self[3] / k])
    }
    #[allow(dead_code)]
    pub fn modulo(&self) -> f32 {
        (*self * *self).sqrt()
    }
    #[allow(dead_code)]
    pub fn normalized(&self) -> Self {
        self.div(self.modulo())
    }
    #[allow(dead_code)]
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;
    fn index<'a>(&'a self, i: usize) -> &'a f32 {
        &self.0[i]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f32 {
        &mut self.0[i]
    }
}

impl Add for Vec4 {
    type Output = Self;
    fn add(self, v2: Self) -> Self {
        Vec4([
            self[0] + v2[0],
            self[1] + v2[1],
            self[2] + v2[2],
            self[3] + v2[3],
        ])
    }
}

impl Neg for Vec4 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec4([-self[0], -self[1], -self[2], -self[3]])
    }
}
impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, v2: Self) -> Self {
        Vec4([
            self[0] - v2[0],
            self[1] - v2[1],
            self[2] - v2[2],
            self[3] - v2[3],
        ])
    }
}

impl Mul for Vec4 {
    type Output = f32;
    fn mul(self, m2: Self) -> f32 {
        self[0] * m2[0] + self[1] * m2[1] + self[2] * m2[2] + self[3] * m2[3]
    }
}
