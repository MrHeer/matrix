use std::ops::{Add, Index, Mul, Sub};

use crate::{Matrix, Vector};

impl<const ROW: usize, const COL: usize> Index<usize> for Matrix<ROW, COL> {
    type Output = Vector<COL>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const ROW: usize, const COL: usize> Add for Matrix<ROW, COL> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .map(|(index, _)| self[index] + rhs[index])
            .collect()
    }
}

impl<const ROW: usize, const COL: usize> Sub for Matrix<ROW, COL> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .map(|(index, _)| self[index] - rhs[index])
            .collect()
    }
}

impl<const ROW: usize, const COL: usize> Mul<f64> for Matrix<ROW, COL> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.scale(rhs)
    }
}

impl<const M: usize, const N: usize, const P: usize> Mul<Matrix<N, P>> for Matrix<M, N> {
    type Output = Matrix<M, P>;

    fn mul(self, rhs: Matrix<N, P>) -> Self::Output {
        self.multiply(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn eq() {
        assert_eq!(matrix::<0, 0>([]), matrix::<0, 0>([]));
        assert_eq!(
            matrix([[2., 1., 2.], [4., 5., 1.]]),
            matrix([[2., 1., 2.], [4., 5., 1.]])
        );
    }

    #[test]
    fn ne() {
        assert_ne!(
            matrix([[2., 1., 2.], [4., 5., 1.]]),
            matrix([[2., 4., 2.], [4., 5., 1.]])
        );
    }

    #[test]
    fn index() {
        let m = matrix([[2., 3., 0.], [4., 8., 1.]]);
        assert_eq!(m[1][0], 4.);
    }

    #[test]
    fn add() {
        let a = matrix([[1., 2., 3.]]);
        let b = matrix([[4., 5., 6.]]);
        let r = a + b;
        assert_eq!(r.round(3), matrix([[5., 7., 9.]]));

        let a = matrix([[4.]]);
        let b = matrix([[5.]]);
        let r = a + b;
        assert_eq!(r.round(3), matrix([[9.]]));

        let a = matrix([[1., 2., 3.], [4., 5., 6.]]);
        let b = matrix([[7., 8., 9.], [10., 11., 12.]]);
        let r = a + b;
        assert_eq!(r.round(3), matrix([[8., 10., 12.], [14., 16., 18.]]));
    }

    #[test]
    fn sub() {
        let a = matrix([[1., 2., 3.]]);
        let b = matrix([[4., 5., 6.]]);
        let r = a - b;
        assert_eq!(r.round(3), matrix([[-3., -3., -3.]]));

        let a = matrix([[4.]]);
        let b = matrix([[5.]]);
        let r = a - b;
        assert_eq!(r.round(3), matrix([[-1.]]));

        let a = matrix([[1., 2., 3.], [4., 5., 6.]]);
        let b = matrix([[7., 8., 9.], [10., 11., 12.]]);
        let r = a - b;
        assert_eq!(r.round(3), matrix([[-6., -6., -6.], [-6., -6., -6.]]));
    }

    #[test]
    fn mul() {
        let a = matrix([[5.], [2.]]);
        let b = matrix([[5., 1.]]);
        let r = a * b;
        assert_eq!(r.round(3), matrix([[25., 5.], [10., 2.]]));

        let a = matrix([[5., 1.]]);
        let b = matrix([[5.], [2.]]);
        let r = a * b;
        assert_eq!(r.round(3), matrix([[27.]]));

        let a = matrix([[3.]]);
        let b = matrix([[4.]]);
        let r = a * b;
        assert_eq!(r.round(3), matrix([[12.]]));

        let a = matrix([[2., 1., 8., 2., 1.], [5., 6., 4., 2., 1.]]);
        let b = matrix([
            [1., 7., 2.],
            [2., 6., 3.],
            [3., 1., 1.],
            [1., 20., 1.],
            [7., 4., 16.],
        ]);
        let r = a * b;
        assert_eq!(r.round(3), matrix([[37., 72., 33.], [38., 119., 50.]]));
    }
}
