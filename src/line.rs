mod fmt;
mod ops;

use crate::{math::first_nonzero_index, vector, Vector};

#[derive(Debug)]
pub struct Line<const DIM: usize> {
    normal_vector: Vector<DIM>,
    constant_term: f64,
    basepoint: Option<Vector<DIM>>,
}

impl Line<2> {
    pub fn new(normal_vector: Option<Vector<2>>, constant_term: Option<f64>) -> Self {
        let normal_vector = normal_vector.unwrap_or(vector([0.; 2]));
        let constant_term = constant_term.unwrap_or(0.);
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
}

#[cfg(test)]
mod tests {
    use crate::vector;

    use super::Line;

    #[test]
    fn new() {
        let line = Line::new(None, None);
        assert_eq!(line.normal_vector, vector([0.; 2]));
        assert_eq!(line.constant_term, 0.);
        assert_eq!(line.basepoint, None);

        let line = Line::new(Some(vector([0., 1.])), Some(3.));
        assert_eq!(line.normal_vector, vector([0., 1.]));
        assert_eq!(line.constant_term, 3.);
        assert_eq!(line.basepoint, Some(vector([0.0, 3.0])));
    }
}
