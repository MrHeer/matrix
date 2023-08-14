mod fmt;
mod iter;
mod ops;

use std::f64::consts::PI;

use crate::{math, round::round_factory};

const ZERO_VECTOR_HAS_NO_NORMALIZE: &str = "Zero vector has no normalize.";

#[derive(Debug, Clone, Copy)]
pub struct Vector<const DIM: usize>([f64; DIM]);

#[derive(Debug, Clone, Copy)]
pub struct Projection<const DIM: usize> {
    pub parallel: Vector<DIM>,
    pub orthogonal: Vector<DIM>,
}

impl<const DIM: usize> From<[f64; DIM]> for Vector<DIM> {
    fn from(arr: [f64; DIM]) -> Self {
        Vector(arr)
    }
}

pub fn vector<const DIM: usize>(arr: [f64; DIM]) -> Vector<DIM> {
    Vector::from(arr)
}

impl<const DIM: usize> Vector<DIM> {
    pub fn dim(&self) -> usize {
        DIM
    }

    pub fn map<F>(self, f: F) -> Vector<DIM>
    where
        F: FnMut(f64) -> f64,
    {
        self.into_iter().map(f).collect()
    }

    pub fn round(self, precision: usize) -> Vector<DIM> {
        let round = round_factory(precision);
        self.map(round)
    }

    pub fn scale(self, scalar: f64) -> Vector<DIM> {
        self.map(|x| x * scalar)
    }

    pub fn magnitude(self) -> f64 {
        let mut sum = 0.;
        self.into_iter().for_each(|x| {
            sum += x.powi(2);
        });
        sum.sqrt()
    }

    pub fn normalize(self) -> Result<Vector<DIM>, String> {
        let magnitude = self.magnitude();
        if magnitude == 0. {
            return Err(String::from(ZERO_VECTOR_HAS_NO_NORMALIZE));
        }
        Ok(self.scale(1. / magnitude))
    }

    pub fn dot(self, other: Vector<DIM>) -> f64 {
        let mut dot_product = 0.;
        self.into_iter()
            .enumerate()
            .for_each(|(index, _)| dot_product += self[index] * other[index]);
        dot_product
    }

    /// return the angle between the two vectors in radian.
    pub fn angle(self, other: Vector<DIM>) -> Result<f64, String> {
        let self_normalize = self.normalize()?;
        let other_normalize = other.normalize()?;
        let dot_product = self_normalize.dot(other_normalize);
        let fixed_prodcut = match dot_product {
            dot_product if dot_product < -1.0 => -1.0,
            dot_product if dot_product > 1.0 => 1.0,
            dot_product => dot_product,
        };

        Ok(fixed_prodcut.acos())
    }

    pub fn is_zero_with_tolerance(self, tolerance: Option<f64>) -> bool {
        math::is_zero_with_tolerance(self.magnitude(), tolerance)
    }

    pub fn is_zero(self) -> bool {
        self.is_zero_with_tolerance(None)
    }

    pub fn is_parallel(self, other: Vector<DIM>) -> bool {
        let angle = self.angle(other);
        match angle {
            Ok(rad) => rad == 0. || rad == PI,
            Err(_) => true,
        }
    }

    pub fn is_orthogonal_with_tolerance(self, other: Vector<DIM>, tolerance: Option<f64>) -> bool {
        math::is_zero_with_tolerance(self.dot(other), tolerance)
    }

    pub fn is_orthogonal(self, other: Vector<DIM>) -> bool {
        self.is_orthogonal_with_tolerance(other, None)
    }

    pub fn project(self, basis: Vector<DIM>) -> Result<Projection<DIM>, String> {
        let u = basis.normalize()?;
        let weight = self.dot(u);
        let parallel = u.scale(weight);
        let orthogonal = self - parallel;
        Ok(Projection {
            parallel,
            orthogonal,
        })
    }
}

impl Vector<3> {
    pub fn cross(self, other: Vector<3>) -> Vector<3> {
        let [x1, y1, z1] = self.0;
        let [x2, y2, z2] = other.0;
        vector([y1 * z2 - y2 * z1, -(x1 * z2 - x2 * z1), x1 * y2 - x2 * y1])
    }

    pub fn area_of_parallelogram(self, other: Vector<3>) -> f64 {
        let cross_product = self.cross(other);
        cross_product.magnitude()
    }

    pub fn area_of_triangle(self, other: Vector<3>) -> f64 {
        self.area_of_parallelogram(other) / 2.
    }
}

#[cfg(test)]
mod tests {
    use super::ZERO_VECTOR_HAS_NO_NORMALIZE;
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
        let u = v.normalize().unwrap().round(3);
        assert_eq!(u, vector([-0.577, 0.577, 0.577]));

        let v = vector([5.581, -2.136]);
        let u = v.normalize().unwrap().round(3);
        assert_eq!(u, vector([0.934, -0.357]));

        let v = vector([1.996, 3.108, -4.554]);
        let u = v.normalize().unwrap().round(3);
        assert_eq!(u, vector([0.34, 0.53, -0.777]));
    }

    #[test]
    fn normalize_zero() {
        let v = vector([0., 0.]);

        assert_eq!(
            v.normalize(),
            Err(String::from(ZERO_VECTOR_HAS_NO_NORMALIZE))
        );
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

    #[test]
    fn checks() {
        let v = vector([0., 0.]);
        let w = vector([-0.1, 0.001]);
        assert_eq!(v.is_zero(), true);
        assert_eq!(w.is_zero(), false);

        let v = vector([-7.579, -7.88]);
        let w = vector([22.737, 23.64]);
        assert_eq!(v.is_parallel(w), true);
        assert_eq!(v.is_orthogonal(w), false);

        let v = vector([-2.029, 9.97, 4.172]);
        let w = vector([-9.231, -6.639, -7.245]);
        assert_eq!(v.is_parallel(w), false);
        assert_eq!(v.is_orthogonal(w), false);

        let v = vector([-2.328, -7.284, -1.214]);
        let w = vector([-1.821, 1.072, -2.94]);
        assert_eq!(v.is_parallel(w), false);
        assert_eq!(v.is_orthogonal(w), true);

        let v = vector([2.118, 4.827]);
        let w = vector([0., 0.]);
        assert_eq!(v.is_parallel(w), true);
        assert_eq!(v.is_orthogonal(w), true);
    }

    #[test]
    fn project() {
        let v = vector([3.039, 1.879]);
        let b = vector([0.825, 2.036]);
        let projection = v.project(b).unwrap();
        assert_eq!(projection.parallel.round(3), vector([1.083, 2.672]));

        let v = vector([-9.88, -3.264, -8.159]);
        let b = vector([-2.155, -9.353, -9.473]);
        let projection = v.project(b).unwrap();
        assert_eq!(
            projection.orthogonal.round(3),
            vector([-8.35, 3.376, -1.434])
        );

        let v = vector([3.009, -6.172, 3.692, -2.51]);
        let b = vector([6.404, -9.144, 2.759, 8.718]);
        let projection = v.project(b).unwrap();
        assert_eq!(
            projection.parallel.round(3),
            vector([1.969, -2.811, 0.848, 2.680])
        );
        assert_eq!(
            projection.orthogonal.round(3),
            vector([1.04, -3.361, 2.844, -5.19])
        );
    }

    #[test]
    fn cross() {
        let v = vector([5., 3., -2.]);
        let w = vector([-1., 0., 3.]);
        let c = v.cross(w);
        assert_eq!(c.round(3), vector([9., -13., 3.]));
        assert_eq!(v.is_orthogonal(c), true);
        assert_eq!(w.is_orthogonal(c), true);

        let v = vector([8.462, 7.893, -8.187]);
        let w = vector([6.984, -5.975, 4.778]);
        assert_eq!(v.cross(w).round(3), vector([-11.205, -97.609, -105.685]));
    }

    #[test]
    fn area_of_parallelogra() {
        let round = round_factory(3);
        let v = vector([-8.987, -9.838, 5.031]);
        let w = vector([-4.268, -1.861, -8.866]);
        assert_eq!(round(v.area_of_parallelogram(w)), 142.122);
    }

    #[test]
    fn area_of_triangle() {
        let round = round_factory(3);
        let v = vector([1.5, 9.547, 3.691]);
        let w = vector([-6.007, 0.124, 5.772]);
        assert_eq!(round(v.area_of_triangle(w)), 42.565);
    }
}
