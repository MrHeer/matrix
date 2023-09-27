use std::ops::{Index, IndexMut};

use crate::equation::Equation;

use super::LinearSystem;

impl<const DIM: usize> Index<usize> for LinearSystem<DIM> {
    type Output = Equation<DIM>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.equations[index]
    }
}

impl<const DIM: usize> IndexMut<usize> for LinearSystem<DIM> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.equations[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{equation::Equation, linear_system::LinearSystem, vector};

    #[test]
    fn index() {
        let p0 = Equation::new(vector([1., 1., 1.]), 1.);
        let p1 = Equation::new(vector([0., 1., 0.]), 2.);
        let p2 = Equation::new(vector([1., 1., -1.]), 3.);
        let p3 = Equation::new(vector([1., 0., -2.]), 2.);

        let s = LinearSystem::new(vec![p0, p1, p2, p3]);

        assert_eq!(s[2], p2);
    }

    #[test]
    fn index_mut() {
        let p0 = Equation::new(vector([1., 1., 1.]), 1.);
        let p1 = Equation::new(vector([0., 1., 0.]), 2.);
        let p2 = Equation::new(vector([1., 1., -1.]), 3.);
        let p3 = Equation::new(vector([1., 0., -2.]), 2.);
        let p4 = Equation::new(vector([-1., 0., -2.]), 1.);

        let mut s = LinearSystem::new(vec![p0, p1, p2, p3]);
        s[2] = p4;

        assert_eq!(s[2], p4);
    }
}
