mod fmt;
mod ops;

use crate::round::round_factory;

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize>([f64; N]);

impl<const N: usize> From<[f64; N]> for Vector<N> {
    fn from(value: [f64; N]) -> Self {
        Vector(value)
    }
}

pub fn vector<const N: usize>(v: [f64; N]) -> Vector<N> {
    Vector::from(v)
}

impl<const N: usize> Vector<N> {
    pub fn dim(&self) -> usize {
        N
    }

    pub fn map<F>(self, f: F) -> Vector<N>
    where
        F: FnMut(f64) -> f64,
    {
        let result_arr = self.0.map(f);
        vector(result_arr)
    }

    pub fn round(self, precision: usize) -> Vector<N> {
        let round = round_factory(precision);
        self.map(round)
    }

    pub fn scale(self, scalar: f64) -> Vector<N> {
        self.map(|x| x * scalar)
    }

    pub fn magnitude(self) -> f64 {
        let mut sum = 0.0_f64;
        self.0.into_iter().for_each(|x| {
            sum += x.powi(2);
        });
        sum.sqrt()
    }

    pub fn normalize<'a>(self) -> Result<Vector<N>, &'a str> {
        let magnitude = self.magnitude();
        if magnitude == 0. {
            return Err("zero vector has no normalize.");
        }
        Ok(self.scale(1. / self.magnitude()))
    }

    pub fn dot(self, other: Vector<N>) -> f64 {
        let mut result = 0.;
        (0..N).for_each(|i| result += self.0[i] * other.0[i]);
        result
    }

    /// return the angle between the two vectors in radian.
    pub fn angle<'a>(self, other: Vector<N>) -> Result<f64, &'a str> {
        Ok(self.normalize()?.dot(other.normalize()?).acos())
    }
}

#[cfg(test)]
mod tests {
    use crate::{math::to_deg, round::round_factory, vector};

    #[test]
    fn dim() {
        let v = vector([2., 3., 3.]);
        assert_eq!(v.dim(), 3);

        let v = vector([]);
        assert_eq!(v.dim(), 0);
    }

    #[test]
    fn map() {
        let v = vector([2., 3., 1.]);
        let double = |x: f64| x * 2.;

        assert_eq!(v.map(double), vector([4., 6., 2.]));
    }

    #[test]
    fn round() {
        let v = vector([1.671, -1.012, -0.318]);

        assert_eq!(v.round(0), vector([2., -1., -0.]));
    }

    #[test]
    fn scale() {
        let v = vector([1.671, -1.012, -0.318]);

        assert_eq!(v.scale(7.41).round(3), vector([12.382, -7.499, -2.356]));
    }

    #[test]
    fn magnitude() {
        let round = round_factory(3);

        let v = vector([3., 4.]);
        assert_eq!(v.magnitude(), 5.);

        let v = vector([-0.221, 7.437]);
        assert_eq!(round(v.magnitude()), 7.44);

        let v = vector([8.813, -1.331, -6.247]);
        assert_eq!(round(v.magnitude()), 10.884);
    }

    #[test]
    fn normalize() {
        let v = vector([-1., 1., 1.]);
        let n = v.normalize().unwrap().round(3);
        assert_eq!(n, vector([-0.577, 0.577, 0.577]));

        let v = vector([5.581, -2.136]);
        let n = v.normalize().unwrap().round(3);
        assert_eq!(n, vector([0.934, -0.357]));

        let v = vector([1.996, 3.108, -4.554]);
        let n = v.normalize().unwrap().round(3);
        assert_eq!(n, vector([0.34, 0.53, -0.777]));
    }

    #[test]
    fn normalize_zero() {
        let v = vector([0., 0.]);

        assert_eq!(v.normalize(), Err("zero vector has no normalize."));
    }

    #[test]
    fn dot() {
        let round = round_factory(3);

        let v = vector([7.887, 4.138]);
        let w = vector([-8.802, 6.776]);
        assert_eq!(round(v.dot(w)), -41.382);

        let v = vector([-5.955, -4.904, -1.874]);
        let w = vector([-4.496, -8.755, 7.103]);
        assert_eq!(round(v.dot(w)), 56.397);
    }

    #[test]
    fn angle() {
        let round = round_factory(3);

        let v = vector([3.183, -7.627]);
        let w = vector([-2.668, 5.319]);
        assert_eq!(round(v.angle(w).unwrap()), 3.072);

        let v = vector([7.35, 0.221, 5.188]);
        let w = vector([2.751, 8.259, 3.985]);
        assert_eq!(round(to_deg(v.angle(w).unwrap())), 60.276);
    }
}
