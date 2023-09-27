use std::fmt::Display;

use super::LinearSystem;

impl<const DIM: usize, const LEN: usize> Display for LinearSystem<DIM, LEN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() == 0 {
            return write!(f, "No Equation");
        }

        writeln!(f, "Linear System:")?;
        for (i, p) in self.0.into_iter().enumerate() {
            writeln!(f, "Equation {}: {}", i + 1, p)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{equation, linear_system, vector};

    #[test]
    fn fmt() {
        let e0 = equation(vector([1., 1., 1.]), 1.);
        let e1 = equation(vector([0., 1., 0.]), 2.);
        let e2 = equation(vector([1., 1., -1.]), 3.);
        let e3 = equation(vector([1., 0., -2.]), 2.);

        let s = linear_system([e0, e1, e2, e3]);
        assert_eq!(
            format!("{}", s),
            r#"Linear System:
Equation 1: x_0 + x_1 + x_2 = 1
Equation 2: x_1 = 2
Equation 3: x_0 + x_1 - x_2 = 3
Equation 4: x_0 - 2x_2 = 2
"#
        );
    }
}
