mod fmt;
mod ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize>(pub [f64; N]);

impl<const N: usize> Vector<N> {
    pub fn dim(&self) -> usize {
        N
    }

    pub fn map<F>(self, f: F) -> Vector<N>
    where
        F: FnMut(f64) -> f64,
    {
        let result_arr = self.0.map(f);
        Vector(result_arr)
    }

    pub fn scale(self, n: f64) -> Vector<N> {
        self.map(|x| x * n)
    }

    pub fn magnitude(self) -> f64 {
        let mut sum = 0.0_f64;
        self.0.into_iter().for_each(|x| {
            sum += x.powi(2);
        });
        sum.sqrt()
    }

    pub fn round(self, precision: usize) -> Vector<N> {
        let factor = 10.0_f64.powi(precision as i32);
        let round = |x: f64| (x * factor).round() / factor;
        self.map(round)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn dim() {
        let v = Vector([2., 3., 3.]);
        assert_eq!(v.dim(), 3);

        let v = Vector([]);
        assert_eq!(v.dim(), 0);
    }

    #[test]
    fn map() {
        let v = Vector([2., 3., 1.]);
        let double = |x: f64| x * 2.;

        assert_eq!(v.map(double), Vector([4., 6., 2.]));
    }

    #[test]
    fn scale() {
        let v = Vector([1.671, -1.012, -0.318]);

        assert_eq!(v.scale(7.41).round(3), Vector([12.382, -7.499, -2.356]));
    }

    #[test]
    fn magnitude() {
        let v = Vector([3.,4.]);

        assert_eq!(v.magnitude(), 5.);
    }

    #[test]
    fn round() {
        let v = Vector([1.671, -1.012, -0.318]);

        assert_eq!(v.round(0), Vector([2., -1., -0.]));
    }
}
