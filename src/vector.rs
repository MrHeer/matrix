mod fmt;
mod ops;

use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize>(pub [T; N]);

impl<T: Mul<Output = T> + Copy, const N: usize> Vector<T, N> {
    pub fn size(&self) -> usize {
        N
    }

    pub fn map<F, U>(self, f: F) -> Vector<U, N>
    where
        F: FnMut(T) -> U,
    {
        let result_arr = self.0.map(f);
        Vector(result_arr)
    }

    pub fn scale(self, n: T) -> Vector<T, N> {
        self.map(|x| x * n)
    }
}

impl<const N: usize> Vector<f64, N> {
    pub fn round(self, precision: usize) -> Vector<f64, N> {
        let factor = 10.0_f64.powi(precision as i32);
        let round = |x: f64| (x * factor).round() / factor;
        self.map(round)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn size() {
        let v = Vector([2, 3, 3]);
        assert_eq!(v.size(), 3);

        let v = Vector::<i32, 0>([]);
        assert_eq!(v.size(), 0);
    }

    #[test]
    fn map() {
        let v = Vector([2, 3, 1]);
        let double = |x: i32| x * 2;

        assert_eq!(v.map(double), Vector([4, 6, 2]));
    }

    #[test]
    fn scale() {
        let v = Vector([1.671, -1.012, -0.318]);

        assert_eq!(v.scale(7.41).round(3), Vector([12.382, -7.499, -2.356]));
    }

    #[test]
    fn round() {
        let v = Vector([1.671, -1.012, -0.318]);

        assert_eq!(v.round(0), Vector([2., -1., -0.]));
    }
}
