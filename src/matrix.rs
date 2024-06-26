mod fmt;
mod iter;
mod ops;

use crate::{round::round_factory, vector, Vector};

const MATRIX_IS_NOT_INVERTIBLE: &str = "Matrix is not invertible.";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const ROW: usize, const COL: usize>([Vector<COL>; ROW]);

impl<const ROW: usize, const COL: usize> From<[[f64; COL]; ROW]> for Matrix<ROW, COL> {
    fn from(value: [[f64; COL]; ROW]) -> Self {
        value.into_iter().map(vector).collect()
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
        self.into_iter().map(|vector| vector.map(&f)).collect()
    }

    pub fn round(&self, precision: usize) -> Self {
        let round = round_factory(precision);
        self.map(round)
    }

    pub fn scale(&self, scalar: f64) -> Self {
        self.map(|x| x * scalar)
    }

    pub fn get_row(&self, row: usize) -> Vector<COL> {
        self[row]
    }

    pub fn get_col(&self, col: usize) -> Vector<ROW> {
        self.into_iter().map(|vector| vector[col]).collect()
    }

    pub fn multiply<const OTHER_COL: usize>(
        &self,
        other: &Matrix<COL, OTHER_COL>,
    ) -> Matrix<ROW, OTHER_COL> {
        let transpose_other = other.transpose();
        self.into_iter()
            .map(|row| {
                transpose_other
                    .into_iter()
                    .map(|other_col| row * other_col)
                    .collect()
            })
            .collect()
    }

    pub fn transpose(&self) -> Matrix<COL, ROW> {
        (0..self.col())
            .map(|col| (0..self.row()).map(|row| self[row][col]).collect())
            .collect()
    }
}

pub fn identity<const N: usize>() -> Matrix<N, N> {
    (0..N)
        .map(|row| (0..N).map(|col| if row == col { 1. } else { 0. }).collect())
        .collect()
}

impl Matrix<1, 1> {
    pub fn inverse(&self) -> Self {
        matrix([[1. / self[0][0]]])
    }
}

impl Matrix<2, 2> {
    pub fn inverse(&self) -> Result<Self, String> {
        let a = self[0][0];
        let b = self[0][1];
        let c = self[1][0];
        let d = self[1][1];

        let det = a * d - b * c;
        let tr = a + d;

        if det == 0. {
            return Err(String::from(MATRIX_IS_NOT_INVERTIBLE));
        }

        Ok((identity() * tr - *self) * (1. / det))
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;
    use crate::matrix::MATRIX_IS_NOT_INVERTIBLE;

    #[test]
    fn transpose() {
        let m = matrix([[5., 4., 1., 7.], [2., 1., 3., 5.]]);
        assert_eq!(
            m.transpose(),
            matrix([[5., 2.], [4., 1.], [1., 3.], [7., 5.]])
        );

        let m = matrix([[5.]]);
        assert_eq!(m.transpose(), matrix([[5.]]));

        let m = matrix([[5., 3., 2.], [7., 1., 4.], [1., 1., 2.], [8., 9., 1.]]);
        assert_eq!(
            m.transpose(),
            matrix([[5., 7., 1., 8.], [3., 1., 1., 9.], [2., 4., 2., 1.]])
        );
    }

    #[test]
    fn identity() {
        assert_eq!(matrix::identity(), matrix([[1.]]));
        assert_eq!(matrix::identity(), matrix([[1., 0.], [0., 1.]]));
        assert_eq!(
            matrix::identity(),
            matrix([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
        );
        assert_eq!(
            matrix::identity(),
            matrix([
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.]
            ])
        );

        let m = matrix([[5., 9., 2., 4.], [3., 8., 5., 6.], [1., 0., 0., 15.]]);
        assert_eq!(m * matrix::identity(), m);
        assert_eq!(matrix::identity() * m, m);
    }

    #[test]
    fn inverse() {
        assert_eq!(matrix([[100.]]).inverse(), matrix([[0.01]]));
        assert_eq!(
            matrix([[4., 5.], [7., 1.]]).inverse().unwrap(),
            matrix([
                [-0.03225806451612903, 0.16129032258064516],
                [0.22580645161290322, -0.12903225806451613]
            ])
        );

        assert_eq!(
            matrix([[4., 2.], [14., 7.]]).inverse(),
            Err(String::from(MATRIX_IS_NOT_INVERTIBLE))
        );
    }
}
