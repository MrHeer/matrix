mod fmt;
mod ops;

use crate::{equation::Equation, math::first_nonzero_index};

const ALL_PLANES_MUST_BE_IN_SAME_DIM_MSG: &str =
    "All planes in the system should live in the same dimension";
const NO_SOLUTIONS_MSG: &str = "No solutions";
const INF_SOLUTIONS_MSG: &str = "Infinitely many solutions";

pub struct LinearSystem<const DIM: usize> {
    equations: Vec<Equation<DIM>>,
}

impl<const DIM: usize> LinearSystem<DIM> {
    pub fn new(equations: Vec<Equation<DIM>>) -> Self {
        LinearSystem { equations }
    }

    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        self.equations.swap(row1, row2);
    }

    pub fn multiply_coefficient_and_row(&mut self, coefficient: f64, row: usize) {
        let equation = self[row];
        let new_equation = Equation::new(
            equation.normal_vector.scale(coefficient),
            equation.constant_term * coefficient,
        );
        self[row] = new_equation;
    }

    pub fn add_multiple_times_row_to_row(
        &mut self,
        coefficient: f64,
        row_to_add: usize,
        row_to_be_added_to: usize,
    ) {
        let to_add_equation = self[row_to_add];
        let to_be_added_to_equation = self[row_to_be_added_to];
        let multipled_to_add_equation = Equation::new(
            to_add_equation.normal_vector.scale(coefficient),
            to_add_equation.constant_term * coefficient,
        );
        self[row_to_be_added_to] = Equation::new(
            multipled_to_add_equation.normal_vector + to_be_added_to_equation.normal_vector,
            multipled_to_add_equation.constant_term + to_be_added_to_equation.constant_term,
        );
    }

    pub fn len(&self) -> usize {
        self.equations.len()
    }

    pub fn indices_of_first_nonzero_terms_in_each_row(&self) -> Vec<Option<usize>> {
        let num_equations = self.len();
        let mut indices = vec![None; num_equations];

        for (i, p) in self.equations.clone().into_iter().enumerate() {
            let index = first_nonzero_index(p.normal_vector).map_or(None, |index| Some(index));
            indices[i] = index;
        }

        return indices;
    }
}

#[cfg(test)]
mod tests {
    use crate::{equation::Equation, linear_system::LinearSystem, vector};

    #[test]
    fn indices_of_first_nonzero_terms_in_each_row() {
        let p0 = Equation::new(vector([1., 1., 1.]), 1.);
        let p1 = Equation::new(vector([0., 1., 0.]), 2.);
        let p2 = Equation::new(vector([1., 1., -1.]), 3.);
        let p3 = Equation::new(vector([1., 0., -2.]), 2.);

        let s = LinearSystem::new(vec![p0, p1, p2, p3]);
        assert_eq!(
            s.indices_of_first_nonzero_terms_in_each_row(),
            vec![Some(0), Some(1), Some(0), Some(0)]
        );
    }

    #[test]
    fn operations() {
        let p0 = Equation::new(vector([1., 1., 1.]), 1.);
        let p1 = Equation::new(vector([0., 1., 0.]), 2.);
        let p2 = Equation::new(vector([1., 1., -1.]), 3.);
        let p3 = Equation::new(vector([1., 0., -2.]), 2.);

        let mut s = LinearSystem::new(vec![p0, p1, p2, p3]);
        s.swap_rows(0, 1);
        assert_eq!(s[0] == p1 && s[1] == p0 && s[2] == p2 && s[3] == p3, true);

        s.swap_rows(1, 3);
        assert_eq!(s[0] == p1 && s[1] == p3 && s[2] == p2 && s[3] == p0, true);

        s.swap_rows(3, 1);
        assert_eq!(s[0] == p1 && s[1] == p0 && s[2] == p2 && s[3] == p3, true);

        s.multiply_coefficient_and_row(1., 0);
        assert_eq!(s[0] == p1 && s[1] == p0 && s[2] == p2 && s[3] == p3, true);

        s.multiply_coefficient_and_row(-1., 2);
        assert_eq!(
            s[0] == p1
                && s[1] == p0
                && s[2] == Equation::new(vector([-1., -1., 1.]), -3.)
                && s[3] == p3,
            true
        );

        s.multiply_coefficient_and_row(10., 1);
        assert_eq!(
            s[0] == p1
                && s[1] == Equation::new(vector([10., 10., 10.]), 10.)
                && s[2] == Equation::new(vector([-1., -1., 1.]), -3.)
                && s[3] == p3,
            true
        );

        s.add_multiple_times_row_to_row(0., 0, 1);
        assert_eq!(
            s[0] == p1
                && s[1] == Equation::new(vector([10., 10., 10.]), 10.)
                && s[2] == Equation::new(vector([-1., -1., 1.]), -3.)
                && s[3] == p3,
            true
        );

        s.add_multiple_times_row_to_row(1., 0, 1);
        assert_eq!(
            s[0] == p1
                && s[1] == Equation::new(vector([10., 11., 10.]), 12.)
                && s[2] == Equation::new(vector([-1., -1., 1.]), -3.)
                && s[3] == p3,
            true
        );

        s.add_multiple_times_row_to_row(-1., 1, 0);
        assert_eq!(
            s[0] == Equation::new(vector([-10., -10., -10.]), -10.)
                && s[1] == Equation::new(vector([10., 11., 10.]), 12.)
                && s[2] == Equation::new(vector([-1., -1., 1.]), -3.)
                && s[3] == p3,
            true
        );
    }
}
