use super::Line;

impl<const DIM: usize> PartialEq for Line<DIM> {
    fn eq(&self, other: &Self) -> bool {
        match (self.basepoint, other.basepoint) {
            (Some(self_basepoint), Some(other_basepoint)) => {
                let connect_vector = self_basepoint - other_basepoint;
                connect_vector.is_orthogonal(&self.normal_vector)
                    && connect_vector.is_orthogonal(&other.normal_vector)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{line::Line, vector};

    #[test]
    fn eq() {
        assert_eq!(
            Line::new(vector([1., 2.]), 2.),
            Line::new(vector([2., 4.]), 4.),
        );
    }

    #[test]
    fn ne() {
        assert_ne!(
            Line::new(vector([0., 0.]), 0.),
            Line::new(vector([0., 0.]), 0.)
        );
        assert_ne!(
            Line::new(vector([1., 0.]), 0.),
            Line::new(vector([0., 0.]), 0.)
        );
        assert_ne!(
            Line::new(vector([1., 2.]), 3.),
            Line::new(vector([1., 2.]), 4.),
        );
    }
}
