use std::fmt::Display;

use super::LinearSystem;

impl<const DIM: usize> Display for LinearSystem<DIM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() == 0 {
            return write!(f, "No Equation");
        }

        writeln!(f, "Linear System:")?;
        for (i, p) in self.equations.clone().into_iter().enumerate() {
            writeln!(f, "Equation {}: {}", i + 1, p)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{equation::Equation, linear_system::LinearSystem, vector};

    #[test]
    fn fmt() {
        let p0 = Equation::new(vector([1., 1., 1.]), 1.);
        let p1 = Equation::new(vector([0., 1., 0.]), 2.);
        let p2 = Equation::new(vector([1., 1., -1.]), 3.);
        let p3 = Equation::new(vector([1., 0., -2.]), 2.);

        let s = LinearSystem::new(vec![p0, p1, p2, p3]);
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
