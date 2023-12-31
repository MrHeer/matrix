use std::fmt::Display;

use crate::math::{first_nonzero_index, is_zero, ne};

use super::Equation;

fn write_coefficient(
    f: &mut std::fmt::Formatter<'_>,
    coefficient: f64,
    is_initial_term: bool,
) -> std::fmt::Result {
    // write sign
    let sign = {
        match (is_initial_term, coefficient.is_sign_positive()) {
            (true, true) => "",
            (true, false) => "-",
            (false, true) => " + ",
            (false, false) => " - ",
        }
    };
    write!(f, "{}", sign)?;

    // write coefficine
    let coefficien = coefficient.abs();
    if ne(coefficien, 1.) {
        coefficien.fmt(f)?;
    }
    Ok(())
}

impl<const DIM: usize> Display for Equation<DIM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let normal_vector = self.normal_vector;
        match first_nonzero_index(normal_vector) {
            Ok(first_index) => {
                for index in first_index..DIM {
                    let coefficient = normal_vector[index];
                    if is_zero(coefficient) {
                        continue;
                    }
                    write_coefficient(f, coefficient, first_index == index)?;
                    write!(f, "x_{}", index)?;
                }
                write!(f, " = ")?;
                self.constant_term.fmt(f)
            }
            Err(err) => write!(f, "{}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{equation, vector};

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", equation(vector([8.3, 0.]), 0.)), "8.3x_0 = 0");
        assert_eq!(
            format!("{}", equation(vector([-5.4, 0.]), 3.)),
            "-5.4x_0 = 3"
        );
        assert_eq!(
            format!("{}", equation(vector([0., 2.9]), 2.3)),
            "2.9x_1 = 2.3"
        );
        assert_eq!(
            format!("{}", equation(vector([0., -8.3]), 0.)),
            "-8.3x_1 = 0"
        );
        assert_eq!(
            format!("{}", equation(vector([-0.12, 2.3]), 5.4)),
            "-0.12x_0 + 2.3x_1 = 5.4"
        );
        assert_eq!(
            format!("{}", equation(vector([5.6, -8.3]), 0.)),
            "5.6x_0 - 8.3x_1 = 0"
        );
        assert_eq!(
            format!("{}", equation(vector([5.6, -1.0]), 0.)),
            "5.6x_0 - x_1 = 0"
        );
        assert_eq!(
            format!("{:.2}", equation(vector([3.231, 0.]), 2.519)),
            "3.23x_0 = 2.52"
        );
    }
}
