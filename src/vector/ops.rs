use std::ops::{Add, Index, Sub};

use crate::Vector;

impl<const DIM: usize> PartialEq for Vector<DIM> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const DIM: usize> Add for Vector<DIM> {
    type Output = Vector<DIM>;

    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .map(|(index, _)| self[index] + rhs[index])
            .collect::<Vector<DIM>>()
    }
}

impl<const DIM: usize> Sub for Vector<DIM> {
    type Output = Vector<DIM>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .map(|(index, _)| self[index] - rhs[index])
            .collect::<Vector<DIM>>()
    }
}

impl<const DIM: usize> Index<usize> for Vector<DIM> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::vector;

    #[test]
    fn eq() {
        assert_eq!(vector([]), vector([]));
        assert_eq!(vector([2., 1., 2.]), vector([2., 1., 2.]));
        assert_ne!(vector([3., 1., 2.]), vector([2., 1., 2.]));
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
    fn index() {
        let v = vector([2., 3., 0.]);
        assert_eq!(v[1], 3.);
    }
}
