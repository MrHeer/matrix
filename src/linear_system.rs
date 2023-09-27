mod fmt;
mod ops;

use crate::{math::first_nonzero_index, space::Space};

const ALL_PLANES_MUST_BE_IN_SAME_DIM_MSG: &str =
    "All planes in the system should live in the same dimension";
const NO_SOLUTIONS_MSG: &str = "No solutions";
const INF_SOLUTIONS_MSG: &str = "Infinitely many solutions";

pub struct LinearSystem<const DIM: usize> {
    spaces: Vec<Space<DIM>>,
}

impl<const DIM: usize> LinearSystem<DIM> {
    pub fn new(spaces: Vec<Space<DIM>>) -> Self {
        LinearSystem { spaces }
    }

    pub fn swap_rows(self, row1: usize, row2: usize) {}

    pub fn multiply_coefficient_and_row(self, coefficient: f64, row: usize) {}

    pub fn add_multiple_times_row_to_row(
        self,
        coefficient: f64,
        row_to_add: usize,
        row_to_be_added_to: usize,
    ) {
    }

    pub fn len(&self) -> usize {
        self.spaces.len()
    }

    pub fn indices_of_first_nonzero_terms_in_each_row(&self) -> Vec<Option<usize>> {
        let num_equations = self.len();
        let mut indices = vec![None; num_equations];

        for (i, p) in self.spaces.clone().into_iter().enumerate() {
            let index = first_nonzero_index(p.normal_vector).map_or(None, |index| Some(index));
            indices[i] = index;
        }

        return indices;
    }
}

#[cfg(test)]
mod tests {
    use crate::{linear_system::LinearSystem, space::Space, vector};

    #[test]
    fn indices_of_first_nonzero_terms_in_each_row() {
        let p0 = Space::new(vector([1., 1., 1.]), 1.);
        let p1 = Space::new(vector([0., 1., 0.]), 2.);
        let p2 = Space::new(vector([1., 1., -1.]), 3.);
        let p3 = Space::new(vector([1., 0., -2.]), 2.);

        let s = LinearSystem::new(vec![p0, p1, p2, p3]);
        assert_eq!(
            s.indices_of_first_nonzero_terms_in_each_row(),
            vec![Some(0), Some(1), Some(0), Some(0)]
        );
    }
}
