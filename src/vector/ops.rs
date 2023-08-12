use std::ops::Add;
use std::ops::Sub;

use crate::{vector, Vector};

impl<const DIM: usize> PartialEq for Vector<DIM> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const DIM: usize> Add for Vector<DIM> {
    type Output = Vector<DIM>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result_arr = self.0;

        (0..DIM).for_each(|i| result_arr[i] = self.0[i] + rhs.0[i]);

        vector(result_arr)
    }
}

impl<const DIM: usize> Sub for Vector<DIM> {
    type Output = Vector<DIM>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result_arr = self.0;

        (0..DIM).for_each(|i| result_arr[i] = self.0[i] - rhs.0[i]);

        vector(result_arr)
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
}
