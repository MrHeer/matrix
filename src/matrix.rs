mod fmt;
mod iter;
mod ops;

use crate::{round::round_factory, vector, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const ROW: usize, const COL: usize>([Vector<COL>; ROW]);

impl<const ROW: usize, const COL: usize> From<[[f64; COL]; ROW]> for Matrix<ROW, COL> {
    fn from(value: [[f64; COL]; ROW]) -> Self {
        value.into_iter().map(|arr| vector(arr)).collect()
    }
}

pub fn matrix<const ROW: usize, const COL: usize>(vectors: [[f64; COL]; ROW]) -> Matrix<ROW, COL> {
    Matrix::from(vectors)
}

impl<const ROW: usize, const COL: usize> Matrix<ROW, COL> {
    pub fn row(&self) -> usize {
        ROW
    }

    pub fn col(&self) -> usize {
        COL
    }

    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        self.into_iter()
            .map(|vector| vector.map(|x| f(x)))
            .collect()
    }

    pub fn round(&self, precision: usize) -> Self {
        let round = round_factory(precision);
        self.map(round)
    }

    pub fn scale(&self, scalar: f64) -> Self {
        self.map(|x| x * scalar)
    }
}
