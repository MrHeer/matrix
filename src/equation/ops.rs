use std::ops::{Add, Sub};

use crate::{equation, math::eq};

use super::Equation;

impl<const DIM: usize> PartialEq for Equation<DIM> {
    fn eq(&self, other: &Self) -> bool {
        match (self.base_point, other.base_point) {
            (Some(self_basepoint), Some(other_basepoint)) => {
                let connect_vector = self_basepoint - other_basepoint;
                connect_vector.is_orthogonal(&self.normal_vector)
                    && connect_vector.is_orthogonal(&other.normal_vector)
            }
            (None, None) => eq(self.constant_term, other.constant_term),
            _ => false,
        }
    }
}

impl<const DIM: usize> Add for Equation<DIM> {
    type Output = Equation<DIM>;

    fn add(self, rhs: Self) -> Self::Output {
        equation(
            self.normal_vector + rhs.normal_vector,
            self.constant_term + rhs.constant_term,
        )
    }
}

impl<const DIM: usize> Sub for Equation<DIM> {
    type Output = Equation<DIM>;

    fn sub(self, rhs: Self) -> Self::Output {
        equation(
            self.normal_vector - rhs.normal_vector,
            self.constant_term - rhs.constant_term,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{equation, vector};

    #[test]
    fn eq() {
        assert_eq!(
            equation(vector([1., 2.]), 2.),
            equation(vector([2., 4.]), 4.),
        );
        assert_eq!(
            equation(vector([0., 0.]), 0.),
            equation(vector([0., 0.]), 0.)
        );
    }

    #[test]
    fn ne() {
        assert_ne!(
            equation(vector([1., 0.]), 0.),
            equation(vector([0., 0.]), 0.)
        );
        assert_ne!(
            equation(vector([1., 2.]), 3.),
            equation(vector([1., 2.]), 4.),
        );
    }

    #[test]
    fn add() {
        let equation_1 = equation(vector([2., 3.]), 2.);
        let equation_2 = equation(vector([1., -1.]), -5.);

        assert_eq!(equation_1 + equation_2, equation(vector([3., 2.]), -3.))
    }

    #[test]
    fn sub() {
        let equation_1 = equation(vector([2., 3.]), 2.);
        let equation_2 = equation(vector([1., -1.]), -5.);

        assert_eq!(equation_1 - equation_2, equation(vector([1., 4.]), 7.))
    }
}
