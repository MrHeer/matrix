use super::Space;

impl<const DIM: usize> PartialEq for Space<DIM> {
    fn eq(&self, other: &Self) -> bool {
        match (self.base_point, other.base_point) {
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
    use crate::{space::Space, vector};

    #[test]
    fn eq() {
        assert_eq!(
            Space::new(vector([1., 2.]), 2.),
            Space::new(vector([2., 4.]), 4.),
        );
    }

    #[test]
    fn ne() {
        assert_ne!(
            Space::new(vector([0., 0.]), 0.),
            Space::new(vector([0., 0.]), 0.)
        );
        assert_ne!(
            Space::new(vector([1., 0.]), 0.),
            Space::new(vector([0., 0.]), 0.)
        );
        assert_ne!(
            Space::new(vector([1., 2.]), 3.),
            Space::new(vector([1., 2.]), 4.),
        );
    }
}
