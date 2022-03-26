use super::vec4::Vec4;
use std::fmt;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Mat4(pub [f32; 16]);

impl fmt::Display for Mat4 {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})\n", self[0], self[1], self[2], self[3])?;
        write!(f, "({}, {}, {}, {})\n", self[4], self[5], self[6], self[7])?;
        write!(
            f,
            "({}, {}, {}, {})\n",
            self[8], self[9], self[10], self[11]
        )?;
        write!(
            f,
            "({}, {}, {}, {})",
            self[12], self[13], self[14], self[15]
        )
    }
}

impl Mat4 {
    pub fn new() -> Self {
        Mat4([0.; 16])
    }
    pub fn identity() -> Self {
        let mut m = Mat4::new();
        m[0] = 1.;
        m[5] = 1.;
        m[10] = 1.;
        m[15] = 1.;
        m
    }
    #[allow(dead_code)]
    pub fn transform(&self, translation: &Vec4, rotation: &Vec4, scale: &Vec4) -> Self {
        self.translate(translation).rotate(rotation).scale(scale)
    }
    #[allow(dead_code)]
    pub fn transpose(&self) -> Self {
        let mut m = self.clone();
        m[4] = m[1];
        m[8] = m[2];
        m[12] = m[3];
        m[9] = m[6];
        m[13] = m[7];
        m[14] = m[11];
        m[1] = self[4];
        m[2] = self[8];
        m[3] = self[12];
        m[6] = self[9];
        m[7] = self[13];
        m[11] = self[14];
        m
    }
    #[allow(dead_code)]
    pub fn rotate_x(&self, angle: f32) -> Self {
        let mut m = Self::identity();
        m[5] = angle.cos();
        m[6] = -angle.sin();
        m[9] = angle.sin();
        m[10] = angle.cos();
        *self * m
    }
    #[allow(dead_code)]
    pub fn rotate_y(&self, angle: f32) -> Self {
        let mut m = Self::identity();
        m[0] = angle.cos();
        m[2] = -angle.sin();
        m[8] = angle.sin();
        m[10] = angle.cos();
        *self * m
    }
    #[allow(dead_code)]
    pub fn rotate_z(&self, angle: f32) -> Self {
        let mut m = Self::identity();
        m[0] = angle.cos();
        m[1] = -angle.sin();
        m[4] = angle.sin();
        m[5] = angle.cos();
        *self * m
    }
    #[allow(dead_code)]
    pub fn rotate(&self, v: &Vec4) -> Self {
        let a = v[0];
        let b = v[1];
        let c = v[2];
        *self * Self::identity().rotate_x(a).rotate_y(b).rotate_z(c)
    }
    #[allow(dead_code)]
    pub fn scale(&self, v: &Vec4) -> Self {
        let mut m = Self::identity();
        m[0] = v[0];
        m[5] = v[1];
        m[10] = v[2];
        *self * m
    }
    #[allow(dead_code)]
    pub fn translate(&self, v: &Vec4) -> Self {
        let mut m = Self::identity();
        m[3] = v[0];
        m[7] = v[1];
        m[11] = v[2];
        *self * m
    }
    fn get_row_indexes_removing_row(row: usize) -> (usize, usize, usize) {
        let mut mi: [usize; 3] = [0, 0, 0];
        for i in 0..3 {
            let increment = if i < row { 0 } else { 1 };
            mi[i] = i + increment;
        }
        (mi[0], mi[1], mi[2])
    }
    #[allow(dead_code)]
    pub fn get<'a>(&'a self, i: usize, j: usize) -> &'a f32 {
        &self[i + j * 4]
    }
    #[allow(dead_code)]
    pub fn inverse(&self) -> Option<Mat4> {
        let determinant = self.determinant();
        // if utils::approx_equal(determinant, 0.0, 1) {
        if determinant == 0.0 {
            super::super::renderer::log(&format!("det {}", determinant));
            return None;
        }
        let ta = self.aggiunta().transpose();
        Some(ta.scalar(1.0 / determinant))
    }
    #[allow(dead_code)]
    pub fn scalar(&self, k: f32) -> Self {
        let mut m = Mat4::new();
        for i in 0..16 {
            m[i] = self[i] * k;
        }
        m
    }

    #[allow(dead_code)]
    pub fn aggiunta(&self) -> Self {
        let mut m = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                m[i + 4 * j] = self.complemento_algebrico(i, j);
            }
        }
        m
    }
    #[allow(dead_code)]
    pub fn complemento_algebrico(&self, _i: usize, _j: usize) -> f32 {
        if _i >= 4 || _j >= 4 {
            return 0.;
        }
        let (i0, i1, i2) = Self::get_row_indexes_removing_row(_i);
        let (j0, j1, j2) = Self::get_row_indexes_removing_row(_j);
        let sign = if (_i + _j) % 2 == 0 {
            1 as f32
        } else {
            -1 as f32
        };
        let mut determinant = 0 as f32;
        determinant += self.get(i0, j0) * self.get(i1, j1) * self.get(i2, j2);
        determinant += self.get(i1, j0) * self.get(i2, j1) * self.get(i0, j2);
        determinant += self.get(i2, j0) * self.get(i0, j1) * self.get(i1, j2);
        determinant -= self.get(i2, j0) * self.get(i1, j1) * self.get(i0, j2);
        determinant -= self.get(i1, j0) * self.get(i0, j1) * self.get(i2, j2);
        determinant -= self.get(i0, j0) * self.get(i2, j1) * self.get(i1, j2);
        determinant * sign
    }
    #[allow(dead_code)]
    pub fn mul(&self, v: &Vec4) -> Vec4 {
        Vec4([
            self[0] * v[0] + self[1] * v[1] + self[2] * v[2] + self[3] * v[3],
            self[4] * v[0] + self[5] * v[1] + self[6] * v[2] + self[7] * v[3],
            self[8] * v[0] + self[9] * v[1] + self[10] * v[2] + self[11] * v[3],
            self[12] * v[0] + self[13] * v[1] + self[14] * v[2] + self[15] * v[3],
        ])
    }
    #[allow(dead_code)]
    pub fn div(&self, k: f32) -> Vec4 {
        Vec4([self[0] / k, self[1] / k, self[2] / k, self[3] / k])
    }
    pub fn data<'a>(&'a self) -> &'a [f32] {
        &self.0[..]
    }
    #[allow(dead_code)]
    pub fn determinant(&self) -> f32 {
        let det = self[0] * self[5] * self[10] * self[15] - self[0] * self[5] * self[11] * self[14]
            + self[0] * self[6] * self[11] * self[13]
            - self[0] * self[6] * self[9] * self[15]
            + self[0] * self[7] * self[9] * self[14]
            - self[0] * self[7] * self[10] * self[13]
            - self[1] * self[4] * self[10] * self[15]
            + self[1] * self[4] * self[11] * self[14]
            + self[1] * self[6] * self[8] * self[15]
            - self[1] * self[6] * self[12] * self[11]
            + self[1] * self[7] * self[8] * self[14]
            - self[1] * self[7] * self[10] * self[12]
            + self[2] * self[7] * self[8] * self[13]
            - self[2] * self[7] * self[9] * self[12]
            + self[2] * self[4] * self[9] * self[14]
            - self[2] * self[4] * self[10] * self[13]
            + self[2] * self[5] * self[10] * self[12]
            - self[2] * self[5] * self[8] * self[14]
            - self[3] * self[4] * self[9] * self[14]
            + self[3] * self[4] * self[10] * self[13]
            + self[3] * self[5] * self[8] * self[14]
            - self[3] * self[5] * self[10] * self[12]
            - self[3] * self[6] * self[8] * self[13]
            + self[3] * self[6] * self[9] * self[12];
        det
    }
}

impl Index<usize> for Mat4 {
    type Output = f32;
    fn index<'a>(&'a self, i: usize) -> &'a f32 {
        &self.0[i]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f32 {
        &mut self.0[i]
    }
}

impl Add for Mat4 {
    type Output = Self;
    fn add(self, m2: Self) -> Self {
        Mat4([
            self[0] + m2[0],
            self[1] + m2[1],
            self[2] + m2[2],
            self[3] + m2[3],
            self[4] + m2[4],
            self[5] + m2[5],
            self[6] + m2[6],
            self[7] + m2[7],
            self[8] + m2[8],
            self[9] + m2[9],
            self[10] + m2[10],
            self[11] + m2[11],
            self[12] + m2[12],
            self[13] + m2[13],
            self[14] + m2[14],
            self[15] + m2[15],
        ])
    }
}

impl Sub for Mat4 {
    type Output = Self;
    fn sub(self, m2: Self) -> Self {
        Mat4([
            self[0] - m2[0],
            self[1] - m2[1],
            self[2] - m2[2],
            self[3] - m2[3],
            self[4] - m2[4],
            self[5] - m2[5],
            self[6] - m2[6],
            self[7] - m2[7],
            self[8] - m2[8],
            self[9] - m2[9],
            self[10] - m2[10],
            self[11] - m2[11],
            self[12] - m2[12],
            self[13] - m2[13],
            self[14] - m2[14],
            self[15] - m2[15],
        ])
    }
}

impl Mul for Mat4 {
    type Output = Self;
    fn mul(self, m2: Self) -> Self {
        Mat4([
            self[0] * m2[0] + self[1] * m2[4] + self[2] * m2[8] + self[3] * m2[12],
            self[0] * m2[1] + self[1] * m2[5] + self[2] * m2[9] + self[3] * m2[13],
            self[0] * m2[2] + self[1] * m2[6] + self[2] * m2[10] + self[3] * m2[14],
            self[0] * m2[3] + self[1] * m2[7] + self[2] * m2[11] + self[3] * m2[15],
            self[4] * m2[0] + self[5] * m2[4] + self[6] * m2[8] + self[7] * m2[12],
            self[4] * m2[1] + self[5] * m2[5] + self[6] * m2[9] + self[7] * m2[13],
            self[4] * m2[2] + self[5] * m2[6] + self[6] * m2[10] + self[7] * m2[14],
            self[4] * m2[3] + self[5] * m2[7] + self[6] * m2[11] + self[7] * m2[15],
            self[8] * m2[0] + self[9] * m2[4] + self[10] * m2[8] + self[11] * m2[12],
            self[8] * m2[1] + self[9] * m2[5] + self[10] * m2[9] + self[11] * m2[13],
            self[8] * m2[2] + self[9] * m2[6] + self[10] * m2[10] + self[11] * m2[14],
            self[8] * m2[3] + self[9] * m2[7] + self[10] * m2[11] + self[11] * m2[15],
            self[12] * m2[0] + self[13] * m2[4] + self[14] * m2[8] + self[15] * m2[12],
            self[12] * m2[1] + self[13] * m2[5] + self[14] * m2[9] + self[15] * m2[13],
            self[12] * m2[2] + self[13] * m2[6] + self[14] * m2[10] + self[15] * m2[14],
            self[12] * m2[3] + self[13] * m2[7] + self[14] * m2[11] + self[15] * m2[15],
        ])
    }
}

#[test]
fn test() {
    let (a, b, c) = Mat4::get_row_indexes_removing_row(0);
    println!("{} {} {}", a, b, c);

    let (a, b, c) = Mat4::get_row_indexes_removing_row(1);
    println!("{} {} {}", a, b, c);

    let (a, b, c) = Mat4::get_row_indexes_removing_row(2);
    println!("{} {} {}", a, b, c);

    let (a, b, c) = Mat4::get_row_indexes_removing_row(3);
    println!("{} {} {}", a, b, c);
}
