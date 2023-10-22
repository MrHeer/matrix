mod fmt;
mod ops;

use crate::{
    equation::Equation,
    math::{first_nonzero_index, is_zero},
    vector, Vector,
};

const NO_SOLUTIONS_MSG: &str = "No solutions";
const INF_SOLUTIONS_MSG: &str = "Infinitely many solutions";

#[derive(Debug, PartialEq)]
pub enum Solution<const DIM: usize> {
    Some(Vector<DIM>),
    None(String),
    Infinity(String),
}

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
        let multiplied_to_add_equation = to_add_equation.scale(coefficient);
        self[row_to_be_added_to] = multiplied_to_add_equation + to_be_added_to_equation
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

    fn clear_coefficient(&mut self, row: usize, col: usize, target_row: usize) {
        let beta = self.coefficient(row, col);
        let gamma = self.coefficient(target_row, col);
        let alpha = -gamma / beta;
        self.add_multiple_times_row_to_row(alpha, row, target_row);
    }

    fn clear_coefficients_above(&mut self, row: usize, col: usize) {
        (0..row).for_each(|current_row| {
            self.clear_coefficient(row, col, current_row);
        });
    }

    fn clear_coefficients_below(&mut self, row: usize, col: usize) {
        let num_equations = LEN;

        (row + 1..num_equations).for_each(|current_row| {
            self.clear_coefficient(row, col, current_row);
        });
    }

    fn compute_triangular_form(&self) -> Self {
        let mut system = self.clone();
        let num_equations = LEN;
        let num_variables = DIM;

        let mut col = 0;
        (0..num_equations).for_each(|row| {
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
        });

        system
    }

    fn scale_row_to_make_coefficient_equal_one(&mut self, row: usize, col: usize) {
        let coefficient = self.coefficient(row, col);
        self.multiply_coefficient_and_row(1. / coefficient, row)
    }

    // Reduced Row-Echelon Form
    fn compute_rref(&self) -> Self {
        let mut tf = self.compute_triangular_form();
        let num_equations = LEN;
        let pivot_indices = tf.indices_of_first_nonzero_terms_in_each_row();

        (0..num_equations).rev().for_each(|row| {
            let col = pivot_indices[row];
            match col {
                Some(col) => {
                    tf.scale_row_to_make_coefficient_equal_one(row, col);
                    tf.clear_coefficients_above(row, col);
                }
                None => (),
            }
        });

        tf
    }

    pub fn compute_solution(&self) -> Solution<DIM> {
        let rref = self.compute_rref();

        if let Some(s) = rref.raise_exception_if_contradictory_equation() {
            return s;
        }

        if let Some(s) = rref.raise_exception_if_too_few_pivots() {
            return s;
        }

        let mut arr = [0.; DIM];
        (0..DIM).for_each(|i| arr[i] = rref.0[i].constant_term);
        Solution::Some(vector(arr))
    }

    fn raise_exception_if_contradictory_equation(&self) -> Option<Solution<DIM>> {
        for equation in self.0 {
            if first_nonzero_index(equation.normal_vector).is_err() {
                let constant_term = equation.constant_term;
                if is_zero(constant_term) == false {
                    return Some(Solution::None(String::from(NO_SOLUTIONS_MSG)));
                }
            }
        }

        None
    }

    fn raise_exception_if_too_few_pivots(&self) -> Option<Solution<DIM>> {
        let pivot_indices = self.indices_of_first_nonzero_terms_in_each_row();
        let num_pivots = {
            pivot_indices.into_iter().fold(0, |sum, index| match index {
                Some(_) => sum + 1,
                None => sum,
            })
        } as usize;
        let num_variables = DIM;

        if num_pivots < num_variables {
            return Some(Solution::Infinity(String::from(INF_SOLUTIONS_MSG)));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        equation,
        linear_system::{linear_system, Solution, INF_SOLUTIONS_MSG, NO_SOLUTIONS_MSG},
        vector,
    };

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

    #[test]
    fn compute_rref() {
        let e1 = equation(vector([1., 1., 1.]), 1.);
        let e2 = equation(vector([0., 1., 1.]), 2.);
        let s = linear_system([e1, e2]);
        let r = s.compute_rref();
        assert_eq!(
            r[0] == equation(vector([1., 0., 0.]), -1.) && r[1] == e2,
            true
        );

        let e1 = equation(vector([1., 1., 1.]), 1.);
        let e2 = equation(vector([1., 1., 1.]), 2.);
        let s = linear_system([e1, e2]);
        let r = s.compute_rref();
        assert_eq!(
            r[0] == e1 && r[1] == equation(vector([0., 0., 0.]), 1.),
            true
        );

        let e1 = equation(vector([1., 1., 1.]), 1.);
        let e2 = equation(vector([0., 1., 0.]), 2.);
        let e3 = equation(vector([1., 1., -1.]), 3.);
        let e4 = equation(vector([1., 0., -2.]), 2.);
        let s = linear_system([e1, e2, e3, e4]);
        let r = s.compute_rref();
        assert_eq!(
            r[0] == equation(vector([1., 0., 0.]), 0.)
                && r[1] == e2
                && r[2] == equation(vector([0., 0., -2.]), 2.)
                && r[3] == equation(vector([0., 0., 0.]), 0.),
            true
        );

        let e1 = equation(vector([0., 1., 1.]), 1.);
        let e2 = equation(vector([1., -1., 1.]), 2.);
        let e3 = equation(vector([1., 2., -5.]), 3.);
        let s = linear_system([e1, e2, e3]);
        let r = s.compute_rref();
        assert_eq!(
            r[0] == equation(vector([1., 0., 0.]), 23. / 9.)
                && r[1] == equation(vector([0., 1., 0.]), 7. / 9.)
                && r[2] == equation(vector([0., 0., 1.]), 2. / 9.),
            true
        );
    }

    #[test]
    fn compute_solution() {
        let e1 = equation(vector([5.862, 1.178, -10.366]), -8.15);
        let e2 = equation(vector([-2.931, -0.589, 5.183]), -4.075);
        let s = linear_system([e1, e2]);
        let solution = s.compute_solution();
        assert_eq!(solution, Solution::None(String::from(NO_SOLUTIONS_MSG)));

        let e1 = equation(vector([8.631, 5.112, -1.816]), -5.113);
        let e2 = equation(vector([4.315, 11.132, -5.27]), -6.775);
        let e3 = equation(vector([-2.158, 3.01, -1.727]), -0.831);
        let s = linear_system([e1, e2, e3]);
        let solution = s.compute_solution();
        assert_eq!(
            solution,
            Solution::Infinity(String::from(INF_SOLUTIONS_MSG))
        );

        let e1 = equation(vector([5.262, 2.739, -9.878]), -3.441);
        let e2 = equation(vector([5.111, 6.358, 7.638]), -2.152);
        let e3 = equation(vector([2.016, -9.924, -1.367]), -9.278);
        let e4 = equation(vector([2.167, -13.543, -18.883]), -10.567);
        let s = linear_system([e1, e2, e3, e4]);
        if let Solution::Some(vec) = s.compute_solution() {
            assert_eq!(
                Solution::Some(vec.round(3)),
                Solution::Some(vector([-1.177, 0.707, -0.083]))
            );
        }
    }
}
