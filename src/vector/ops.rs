use std::ops::Add;
use std::ops::Sub;

use crate::Vector;

impl<T: PartialEq, const N: usize> PartialEq for Vector<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Add<Output = T> + Copy, const N: usize> Add for &Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let self_arr = self.0;
        let rhs_arr = rhs.0;
        let mut result_arr: [T; N] = self.0;

        (0..N).for_each(|i| result_arr[i] = self_arr[i] + rhs_arr[i]);

        Vector(result_arr)
    }
}

impl<T: Sub<Output = T> + Copy, const N: usize> Sub for &Vector<T, N> {
    type Output = Vector<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let self_arr = self.0;
        let rhs_arr = rhs.0;
        let mut result_arr: [T; N] = self.0;

        (0..N).for_each(|i| result_arr[i] = self_arr[i] - rhs_arr[i]);

        Vector(result_arr)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn eq() {
        assert_eq!(Vector::<i32, 0>([]), Vector::<i32, 0>([]));
        assert_eq!(Vector([2, 1, 2]), Vector([2, 1, 2]));
        assert_ne!(Vector([3, 1, 2]), Vector([2, 1, 2]));
    }

    #[test]
    fn ne() {
        assert_ne!(Vector([3, 1, 2]), Vector([2, 1, 2]));
    }

    #[test]
    fn add() {
        let a = Vector([8.218, -9.341]);
        let b = Vector([-1.129, 2.111]);
        let r = &a + &b;

        let round = |x: f64| (x * 1000.0).round() / 1000.0;

        assert_eq!(r.map(round), Vector([7.089, -7.23]))
    }

    #[test]
    fn sub() {
        let a = Vector([7.119, 8.215]);
        let b = Vector([-8.223, 0.878]);
        let r = &a - &b;

        let round = |x: f64| (x * 1000.0).round() / 1000.0;

        assert_eq!(r.map(round), Vector([15.342, 7.337]))
    }
}
