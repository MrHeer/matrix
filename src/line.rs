mod fmt;
pub mod intersection;
mod ops;

use crate::{math::first_nonzero_index, round::round_factory, vector, Vector};

use self::intersection::Intersection;

#[derive(Debug)]
pub struct Line<const DIM: usize> {
    normal_vector: Vector<DIM>,
    constant_term: f64,
    basepoint: Option<Vector<DIM>>,
}

impl Line<2> {
    pub fn new(normal_vector: Vector<2>, constant_term: f64) -> Self {
        let basepoint = Self::build_basepoint(normal_vector, constant_term);
        Line {
            normal_vector,
            constant_term,
            basepoint,
        }
    }

    fn build_basepoint(normal_vector: Vector<2>, constant_term: f64) -> Option<Vector<2>> {
        match first_nonzero_index(normal_vector) {
            Ok(initial_index) => {
                let mut basepoint_coords = [0.; 2];
                let initial_coefficient = normal_vector[initial_index];
                basepoint_coords[initial_index] = constant_term / initial_coefficient;
                Some(vector(basepoint_coords))
            }
            _ => None,
        }
    }

    pub fn round(&self, precision: usize) -> Line<2> {
        let round = &round_factory(precision);
        let basepoint = match self.basepoint {
            Some(basepoint) => Some(basepoint.map(round)),
            None => None,
        };
        Line {
            normal_vector: self.normal_vector.map(round),
            constant_term: round(self.constant_term),
            basepoint,
        }
    }

    pub fn is_parallel(&self, other: &Line<2>) -> bool {
        self.normal_vector.is_parallel(&other.normal_vector)
    }

    pub fn intersect(&self, other: &Line<2>) -> Intersection<2> {
        use Intersection::*;

        if self == other {
            return Infinity;
        }

        if self.is_parallel(other) {
            return None;
        }

        let a = self.normal_vector[0];
        let b = self.normal_vector[1];
        let k1 = self.constant_term;
        let c = other.normal_vector[0];
        let d = other.normal_vector[1];
        let k2 = other.constant_term;
        let divsion = a * d - b * c;
        Some(vector([
            (d * k1 - b * k2) / divsion,
            (-c * k1 + a * k2) / divsion,
        ]))
    }
}

#[cfg(test)]
mod tests {
    use crate::vector;

    use super::Line;

    #[test]
    fn new() {
        let line = Line::new(vector([0., 1.]), 3.);
        assert_eq!(line.normal_vector, vector([0., 1.]));
        assert_eq!(line.constant_term, 3.);
        assert_eq!(line.basepoint, Some(vector([0.0, 3.0])));
    }

    #[test]
    fn round() {
        let line = Line::new(vector([0.3837, 1.3212]), 12.4837);
        assert_eq!(
            line.round(3),
            Line {
                normal_vector: vector([0.384, 1.321]),
                constant_term: 12.484,
                basepoint: Some(vector([32.535, 0.0]))
            }
        );
    }

    #[test]
    fn is_parallel() {
        let line_1 = Line::new(vector([0., 1.]), 3.);
        let line_2 = Line::new(vector([0., 2.]), 6.);
        assert_eq!(line_1.is_parallel(&line_2), true);

        let line_1 = Line::new(vector([2., 1.]), 3.);
        let line_2 = Line::new(vector([1., 2.]), 3.);
        assert_eq!(line_1.is_parallel(&line_2), false);
    }

    #[test]
    fn intersect() {
        use super::Intersection::*;

        let line_1 = Line::new(vector([4.046, 2.836]), 1.21);
        let line_2 = Line::new(vector([10.115, 7.09]), 3.025);
        assert_eq!(line_1.intersect(&line_2), Infinity);

        let line_1 = Line::new(vector([7.204, 3.182]), 8.68);
        let line_2 = Line::new(vector([8.172, 4.114]), 9.883);
        assert_eq!(
            line_1.intersect(&line_2).unwrap().round(3),
            vector([1.173, 0.073])
        );

        let line_1 = Line::new(vector([1.182, 5.562]), 6.744);
        let line_2 = Line::new(vector([1.773, 8.343]), 9.525);
        assert_eq!(line_1.intersect(&line_2), None);
    }
}
