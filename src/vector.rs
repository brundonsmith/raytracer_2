use std::{
    fmt::{Display, Write},
    ops::{Add, Div, Index, Mul, Sub},
};

use crate::{matrix::Matrix, utils::collect_arr};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector<const N: usize> {
    data: [f32; N],
}

impl<const N: usize> Vector<N> {
    pub fn new(arr: [f32; N]) -> Self {
        arr.into()
    }

    pub fn x(self) -> f32 {
        self.data[0]
    }

    pub fn y(self) -> f32 {
        self.data[1]
    }

    pub fn z(self) -> f32 {
        self.data[2]
    }

    pub fn column_matrix(self) -> Matrix<N, 1> {
        let mut res = [[0.0; 1]; N];

        for (index, n) in self.data.into_iter().map(|n| [n]).enumerate() {
            res[index] = n;
        }

        res.into()
    }

    pub fn row_matrix(self) -> Matrix<1, N> {
        [self.data].into()
    }

    pub fn magnitude(self) -> f32 {
        self.data.into_iter().map(|n| n * n).sum::<f32>().sqrt()
    }

    pub fn normalized(self) -> Vector<N> {
        self / self.magnitude()
    }

    pub fn dot(self, other: Vector<N>) -> f32 {
        self.data
            .into_iter()
            .zip(other.data.into_iter())
            .map(|(s, o)| s * o)
            .sum()
    }

    pub fn cross(self, other: Vector<N>) -> Vector<N> {
        let mut res = [0.0; N];

        for i in 0..N {
            res[i] = self.data[(i + 1) % N] * other.data[(i + 2) % N]
                - self.data[(i + 2) % N] * other.data[(i + 1) % N];
        }

        res.into()
    }

    pub fn angle_to(self, other: Vector<N>) -> f32 {
        self.normalized().dot(other.normalized()).acos()
    }

    pub fn bounce_along(self, normal: Vector<N>) -> Vector<N> {
        let normal = normal.normalized();
        self - normal * self.dot(normal.normalized()) * 2.0
    }
}

impl<const N: usize> Display for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;
        for n in 0..N {
            if n > 0 {
                f.write_str(", ")?;
            }

            f.write_fmt(format_args!("{}", self.data[n]))?;
        }
        f.write_char(')')
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize> Add<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn add(self, rhs: Vector<N>) -> Self::Output {
        collect_arr(
            self.data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(l, r)| l + r),
        )
        .into()
    }
}

impl<const N: usize> Sub<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn sub(self, rhs: Vector<N>) -> Self::Output {
        collect_arr(
            self.data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(l, r)| l - r),
        )
        .into()
    }
}

impl<const N: usize> Mul<f32> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: f32) -> Self::Output {
        collect_arr(self.data.into_iter().map(|n| n * rhs)).into()
    }
}

impl<const N: usize> Mul<Vector<N>> for f32 {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        rhs * self
    }
}

impl<const N: usize> Div<f32> for Vector<N> {
    type Output = Vector<N>;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl<const N: usize> From<[f32; N]> for Vector<N> {
    fn from(value: [f32; N]) -> Self {
        Self { data: value }
    }
}

impl<const N: usize> From<Matrix<N, 1>> for Vector<N> {
    fn from(value: Matrix<N, 1>) -> Self {
        collect_arr((0..N).map(|i| value[i][0])).into()
    }
}
