mod fmt;
mod ops;

use crate::{
    equation::Equation,
    math::{first_nonzero_index, is_zero},
};

// const ALL_PLANES_MUST_BE_IN_SAME_DIM_MSG: &str =
//     "All planes in the system should live in the same dimension";
// const NO_SOLUTIONS_MSG: &str = "No solutions";
// const INF_SOLUTIONS_MSG: &str = "Infinitely many solutions";

#[derive(Clone)]
pub struct LinearSystem<const DIM: usize, const LEN: usize>([Equation<DIM>; LEN]);

impl<const DIM: usize, const LEN: usize> From<[Equation<DIM>; LEN]> for LinearSystem<DIM, LEN> {
    fn from(arr: [Equation<DIM>; LEN]) -> Self {
        LinearSystem(arr)
    }
}

pub fn linear_system<const DIM: usize, const LEN: usize>(
    arr: [Equation<DIM>; LEN],
) -> LinearSystem<DIM, LEN> {
    LinearSystem::from(arr)
}

impl<const DIM: usize, const LEN: usize> LinearSystem<DIM, LEN> {
    fn coefficient(&self, row: usize, col: usize) -> f64 {
        self[row].normal_vector[col]
    }

    fn swap_rows(&mut self, row1: usize, row2: usize) {
        self.0.swap(row1, row2);
    }

    fn multiply_coefficient_and_row(&mut self, coefficient: f64, row: usize) {
        self[row] = self[row].scale(coefficient);
    }

    fn add_multiple_times_row_to_row(
        &mut self,
        coefficient: f64,
        row_to_add: usize,
        row_to_be_added_to: usize,
    ) {
        let to_add_equation = self[row_to_add];
        let to_be_added_to_equation = self[row_to_be_added_to];
        let multipled_to_add_equation = to_add_equation.scale(coefficient);
        self[row_to_be_added_to] = multipled_to_add_equation + to_be_added_to_equation
    }

    pub fn len(&self) -> usize {
        LEN
    }

    pub fn dim(&self) -> usize {
        DIM
    }

    fn indices_of_first_nonzero_terms_in_each_row(&self) -> [Option<usize>; LEN] {
        self.0
            .map(|e| first_nonzero_index(e.normal_vector).map_or(None, |index| Some(index)))
    }

    fn swap_with_row_below_for_nonzero_coefficient_if_able(
        &mut self,
        row: usize,
        col: usize,
    ) -> bool {
        let num_equations = LEN;

        for current_row in row + 1..num_equations {
            let coefficient = self.coefficient(current_row, col);
            if is_zero(coefficient) == false {
                self.swap_rows(row, current_row);
                return true;
            }
        }
        false
    }

    fn clear_coefficients_below(&mut self, row: usize, col: usize) {
        let num_equations = LEN;
        let beta = self.coefficient(row, col);

        for current_row in row + 1..num_equations {
            let normal_vector = self[current_row].normal_vector;
            let gamma = normal_vector[col];
            let alpha = -gamma / beta;
            self.add_multiple_times_row_to_row(alpha, row, current_row);
        }
    }

    fn compute_triangular_form(&self) -> Self {
        let mut system = self.clone();
        let num_equations = LEN;
        let num_variables = DIM;

        let mut col = 0;
        for row in 0..num_equations - 1 {
            while col < num_variables {
                let coefficient = system.coefficient(row, col);
                if is_zero(coefficient) {
                    let swap_succeeded =
                        system.swap_with_row_below_for_nonzero_coefficient_if_able(row, col);
                    if swap_succeeded == false {
                        col += 1;
                        continue;
                    }
                }
                system.clear_coefficients_below(row, col);
                col += 1;
                break;
            }
        }
        system
    }
}

#[cfg(test)]
mod tests {
    use crate::{equation, linear_system, vector};

    #[test]
    fn indices_of_first_nonzero_terms_in_each_row() {
        let e0 = equation(vector([1., 1., 1.]), 1.);
        let e1 = equation(vector([0., 1., 0.]), 2.);
        let e2 = equation(vector([1., 1., -1.]), 3.);
        let e3 = equation(vector([1., 0., -2.]), 2.);

        let s = linear_system([e0, e1, e2, e3]);
        assert_eq!(
            s.indices_of_first_nonzero_terms_in_each_row(),
            [Some(0), Some(1), Some(0), Some(0)]
        );
    }

    #[test]
    fn operations() {
        let e0 = equation(vector([1., 1., 1.]), 1.);
        let e1 = equation(vector([0., 1., 0.]), 2.);
        let e2 = equation(vector([1., 1., -1.]), 3.);
        let e3 = equation(vector([1., 0., -2.]), 2.);

        let mut s = linear_system([e0, e1, e2, e3]);
        s.swap_rows(0, 1);
        assert_eq!(s[0] == e1 && s[1] == e0 && s[2] == e2 && s[3] == e3, true);

        s.swap_rows(1, 3);
        assert_eq!(s[0] == e1 && s[1] == e3 && s[2] == e2 && s[3] == e0, true);

        s.swap_rows(3, 1);
        assert_eq!(s[0] == e1 && s[1] == e0 && s[2] == e2 && s[3] == e3, true);

        s.multiply_coefficient_and_row(1., 0);
        assert_eq!(s[0] == e1 && s[1] == e0 && s[2] == e2 && s[3] == e3, true);

        s.multiply_coefficient_and_row(-1., 2);
        assert_eq!(
            s[0] == e1 && s[1] == e0 && s[2] == equation(vector([-1., -1., 1.]), -3.) && s[3] == e3,
            true
        );

        s.multiply_coefficient_and_row(10., 1);
        assert_eq!(
            s[0] == e1
                && s[1] == equation(vector([10., 10., 10.]), 10.)
                && s[2] == equation(vector([-1., -1., 1.]), -3.)
                && s[3] == e3,
            true
        );

        s.add_multiple_times_row_to_row(0., 0, 1);
        assert_eq!(
            s[0] == e1
                && s[1] == equation(vector([10., 10., 10.]), 10.)
                && s[2] == equation(vector([-1., -1., 1.]), -3.)
                && s[3] == e3,
            true
        );

        s.add_multiple_times_row_to_row(1., 0, 1);
        assert_eq!(
            s[0] == e1
                && s[1] == equation(vector([10., 11., 10.]), 12.)
                && s[2] == equation(vector([-1., -1., 1.]), -3.)
                && s[3] == e3,
            true
        );

        s.add_multiple_times_row_to_row(-1., 1, 0);
        assert_eq!(
            s[0] == equation(vector([-10., -10., -10.]), -10.)
                && s[1] == equation(vector([10., 11., 10.]), 12.)
                && s[2] == equation(vector([-1., -1., 1.]), -3.)
                && s[3] == e3,
            true
        );
    }

    #[test]
    fn compute_triangular_form() {
        let e1 = equation(vector([1., 1., 1.]), 1.);
        let e2 = equation(vector([0., 1., 1.]), 2.);
        let s = linear_system([e1, e2]);
        let t = s.compute_triangular_form();
        assert_eq!(t[0] == e1 && t[1] == e2, true);

        let e1 = equation(vector([1., 1., 1.]), 1.);
        let e2 = equation(vector([1., 1., 1.]), 2.);
        let s = linear_system([e1, e2]);
        let t = s.compute_triangular_form();
        println!("{}, {}", t[1].normal_vector, t[1].constant_term);
        assert_eq!(
            t[0] == e1 && t[1] == equation(vector([0., 0., 0.]), 1.),
            true
        );

        let e1 = equation(vector([1., 1., 1.]), 1.);
        let e2 = equation(vector([0., 1., 0.]), 2.);
        let e3 = equation(vector([1., 1., -1.]), 3.);
        let e4 = equation(vector([1., 0., -2.]), 2.);
        let s = linear_system([e1, e2, e3, e4]);
        let t = s.compute_triangular_form();
        assert_eq!(
            t[0] == e1
                && t[1] == e2
                && t[2] == equation(vector([0., 0., -2.]), 2.)
                && t[3] == equation(vector([0., 0., 0.]), 0.),
            true
        );

        let e1 = equation(vector([0., 1., 1.]), 1.);
        let e2 = equation(vector([1., -1., 1.]), 2.);
        let e3 = equation(vector([1., 2., -5.]), 3.);
        let s = linear_system([e1, e2, e3]);
        let t = s.compute_triangular_form();
        assert_eq!(
            t[0] == equation(vector([1., -1., 1.]), 2.)
                && t[1] == equation(vector([0., 1., 1.]), 1.)
                && t[2] == equation(vector([0., 0., -9.]), -2.),
            true
        );
    }
}
