use super::Line;

impl<const DIM: usize> PartialEq for Line<DIM> {
    fn eq(&self, other: &Self) -> bool {
        match (self.basepoint, other.basepoint) {
            (Some(self_basepoint), Some(other_basepoint)) => {
                let connect_vector = self_basepoint - other_basepoint;
                connect_vector.is_orthogonal(self.normal_vector)
                    && connect_vector.is_orthogonal(other.normal_vector)
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
            Line::new(Some(vector([1., 2.])), Some(2.)),
            Line::new(Some(vector([2., 4.])), Some(4.)),
        );
    }

    #[test]
    fn ne() {
        assert_ne!(Line::new(None, None), Line::new(None, None));
        assert_ne!(
            Line::new(Some(vector([1., 0.])), None),
            Line::new(None, None),
        );
        assert_ne!(
            Line::new(Some(vector([1., 2.])), Some(3.)),
            Line::new(Some(vector([1., 2.])), Some(4.)),
        );
    }
}
