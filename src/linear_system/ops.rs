use std::ops::{Index, IndexMut};

use crate::equation::Equation;

use super::LinearSystem;

impl<const DIM: usize, const LEN: usize> Index<usize> for LinearSystem<DIM, LEN> {
    type Output = Equation<DIM>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const DIM: usize, const LEN: usize> IndexMut<usize> for LinearSystem<DIM, LEN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{equation, linear_system, vector};

    #[test]
    fn index() {
        let e0 = equation(vector([1., 1., 1.]), 1.);
        let e1 = equation(vector([0., 1., 0.]), 2.);
        let e2 = equation(vector([1., 1., -1.]), 3.);
        let e3 = equation(vector([1., 0., -2.]), 2.);

        let s = linear_system([e0, e1, e2, e3]);

        assert_eq!(s[2], e2);
    }

    #[test]
    fn index_mut() {
        let e0 = equation(vector([1., 1., 1.]), 1.);
        let e1 = equation(vector([0., 1., 0.]), 2.);
        let e2 = equation(vector([1., 1., -1.]), 3.);
        let e3 = equation(vector([1., 0., -2.]), 2.);
        let e4 = equation(vector([-1., 0., -2.]), 1.);

        let mut s = linear_system([e0, e1, e2, e3]);
        s[2] = e4;

        assert_eq!(s[2], e4);
    }
}
