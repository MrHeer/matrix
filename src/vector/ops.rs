use std::ops::{Add, Index, Mul, Sub};

use crate::Vector;

impl<const DIM: usize> Index<usize> for Vector<DIM> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const DIM: usize> Add for Vector<DIM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .map(|(index, _)| self[index] + rhs[index])
            .collect()
    }
}

impl<const DIM: usize> Sub for Vector<DIM> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .map(|(index, _)| self[index] - rhs[index])
            .collect()
    }
}

impl<const DIM: usize> Mul<f64> for Vector<DIM> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.scale(rhs)
    }
}

impl<const DIM: usize> Mul for Vector<DIM> {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{round::round_factory, vector};

    #[test]
    fn index() {
        let v = vector([2., 3., 0.]);
        assert_eq!(v[1], 3.);
    }

    #[test]
    fn eq() {
        assert_eq!(vector([]), vector([]));
        assert_eq!(vector([2., 1., 2.]), vector([2., 1., 2.]));
    }

    #[test]
    fn ne() {
        assert_ne!(vector([3., 1., 2.]), vector([2., 1., 2.]));
    }

    #[test]
    fn add() {
        let a = vector([8.218, -9.341]);
        let b = vector([-1.129, 2.111]);
        let r = a + b;
        assert_eq!(r.round(3), vector([7.089, -7.23]));
    }

    #[test]
    fn sub() {
        let a = vector([7.119, 8.215]);
        let b = vector([-8.223, 0.878]);
        let r = a - b;
        assert_eq!(r.round(3), vector([15.342, 7.337]));
    }

    #[test]
    fn scale() {
        let v = vector([1.671, -1.012, -0.318]);

        assert_eq!((v * 7.41).round(3), vector([12.382, -7.499, -2.356]));
    }

    #[test]
    fn dot() {
        let round = round_factory(3);

        let v = vector([7.887, 4.138]);
        let w = vector([-8.802, 6.776]);
        assert_eq!(round(v * w), -41.382);

        let v = vector([-5.955, -4.904, -1.874]);
        let w = vector([-4.496, -8.755, 7.103]);
        assert_eq!(round(v.dot(&w)), 56.397);
    }
}
