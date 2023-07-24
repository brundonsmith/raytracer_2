use std::ops::{Index, Mul};

use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f32; C]; R],
}

pub type TransformationMatrix = Matrix<4, 4>;

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub const IDENTITY_4X4: Matrix<4, 4> = Matrix::identity();
    pub const IDENTITY_3X4: Matrix<3, 4> = Matrix::identity();
    pub const IDENTITY_2X3: Matrix<2, 3> = Matrix::identity();

    pub const fn identity() -> Self {
        let mut res = [[0.0; C]; R];
        let mut i = 0;

        while i < R {
            res[i][i] = 1.0;
            i += 1;
        }

        Self { data: res }
    }

    pub fn translate(mut self, vec: Vector<R>) -> Self {
        for r in 0..R {
            self.data[r][C - 1] += vec[r];
        }

        self
    }

    pub fn scale(mut self, vec: Vector<R>) -> Self {
        for r in 0..R {
            self.data[r][r] *= vec[r];
        }

        self
    }
}

impl<const R: usize, const C: usize> Index<usize> for Matrix<R, C> {
    type Output = [f32; C];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const R1: usize, const C1: usize, const C2: usize> Mul<Matrix<C1, C2>> for Matrix<R1, C1> {
    type Output = Matrix<R1, C2>;

    fn mul(self, rhs: Matrix<C1, C2>) -> Self::Output {
        let mut res = [[0.0; C2]; R1];

        for (index, row) in self.data.into_iter().enumerate() {
            for c2 in 0..C2 {
                res[index][c2] = row
                    .into_iter()
                    .zip(rhs.data.into_iter().map(|r| r[c2]))
                    .map(|(l, r)| l * r)
                    .sum();
            }
        }

        res.into()
    }
}

impl<const R1: usize, const C1: usize> Mul<Vector<C1>> for Matrix<R1, C1> {
    type Output = Vector<R1>;

    fn mul(self, rhs: Vector<C1>) -> Self::Output {
        (self * rhs.column_matrix()).into()
    }
}

impl<const R: usize, const C: usize> From<[[f32; C]; R]> for Matrix<R, C> {
    fn from(value: [[f32; C]; R]) -> Self {
        Self { data: value }
    }
}
