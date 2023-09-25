mod fmt;
mod ops;

use crate::{math::first_nonzero_index, round::round_factory, vector, Vector};

#[derive(Debug)]
pub struct Space<const DIM: usize> {
    pub(crate) normal_vector: Vector<DIM>,
    pub(crate) constant_term: f64,
    pub(crate) base_point: Option<Vector<DIM>>,
}

impl<const DIM: usize> Space<DIM> {
    pub fn new(normal_vector: Vector<DIM>, constant_term: f64) -> Self {
        let base_point = Self::build_base_point(normal_vector, constant_term);
        Space {
            normal_vector,
            constant_term,
            base_point,
        }
    }

    fn build_base_point(normal_vector: Vector<DIM>, constant_term: f64) -> Option<Vector<DIM>> {
        match first_nonzero_index(normal_vector) {
            Ok(initial_index) => {
                let mut base_point_coords = [0.; DIM];
                let initial_coefficient = normal_vector[initial_index];
                base_point_coords[initial_index] = constant_term / initial_coefficient;
                Some(vector(base_point_coords))
            }
            _ => None,
        }
    }

    pub fn round(&self, precision: usize) -> Space<DIM> {
        let round = &round_factory(precision);
        let base_point = match self.base_point {
            Some(base_point) => Some(base_point.map(round)),
            None => None,
        };
        Space {
            normal_vector: self.normal_vector.map(round),
            constant_term: round(self.constant_term),
            base_point,
        }
    }

    pub fn is_parallel(&self, other: &Space<DIM>) -> bool {
        self.normal_vector.is_parallel(&other.normal_vector)
    }
}

#[cfg(test)]
mod tests {
    use crate::vector;

    use super::Space;

    #[test]
    fn new() {
        let line = Space::new(vector([0., 1.]), 3.);
        assert_eq!(line.normal_vector, vector([0., 1.]));
        assert_eq!(line.constant_term, 3.);
        assert_eq!(line.base_point, Some(vector([0.0, 3.0])));
    }

    #[test]
    fn round() {
        let line = Space::new(vector([0.3837, 1.3212]), 12.4837);
        assert_eq!(
            line.round(3),
            Space {
                normal_vector: vector([0.384, 1.321]),
                constant_term: 12.484,
                base_point: Some(vector([32.535, 0.0]))
            }
        );
    }

    #[test]
    fn is_parallel() {
        let line_1 = Space::new(vector([0., 1.]), 3.);
        let line_2 = Space::new(vector([0., 2.]), 6.);
        assert_eq!(line_1.is_parallel(&line_2), true);

        let line_1 = Space::new(vector([2., 1.]), 3.);
        let line_2 = Space::new(vector([1., 2.]), 3.);
        assert_eq!(line_1.is_parallel(&line_2), false);

        let planes_1 = Space::new(vector([-0.412, 3.806, 0.728]), -3.46);
        let planes_2 = Space::new(vector([1.03, -9.515, -1.82]), 8.65);
        assert_eq!(planes_1, planes_2);
        assert_eq!(planes_1.is_parallel(&planes_2), true);

        let planes_1 = Space::new(vector([2.611, 5.518, 0.283]), 4.6);
        let planes_2 = Space::new(vector([7.715, 8.306, 5.342]), 3.76);
        assert_ne!(planes_1, planes_2);
        assert_eq!(planes_1.is_parallel(&planes_2), false);

        let planes_1 = Space::new(vector([-7.926, 8.625, -7.212]), -7.952);
        let planes_2 = Space::new(vector([-2.642, 2.875, -2.404]), -2.443);
        assert_ne!(planes_1, planes_2);
        assert_eq!(planes_1.is_parallel(&planes_2), true);
    }
}
