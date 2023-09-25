use crate::{space::Space, vector, Vector};

#[derive(Debug, PartialEq)]
pub enum Intersection {
    Some(Vector<2>),
    None,
    Infinity,
}

impl Intersection {
    pub fn unwrap(self) -> Vector<2> {
        use Intersection::*;
        match self {
            Some(val) => val,
            None => panic!("called `Intersection::unwrap()` on a `None` value"),
            Infinity => panic!("called `Intersection::unwrap()` on a `Infinity` value"),
        }
    }
}

pub type Line = Space<2>;

impl Line {
    pub fn intersect(&self, other: &Line) -> Intersection {
        use Intersection::*;

        if self == other {
            return Infinity;
        }

        if self.is_parallel(other) {
            return None;
        }

        let a = self.normal_vector[0];
        let b = self.normal_vector[1];
        let k1 = self.constant_term;
        let c = other.normal_vector[0];
        let d = other.normal_vector[1];
        let k2 = other.constant_term;
        let divsion = a * d - b * c;
        Some(vector([
            (d * k1 - b * k2) / divsion,
            (-c * k1 + a * k2) / divsion,
        ]))
    }
}

#[cfg(test)]
mod tests {
    use crate::vector;

    use super::Space;

    #[test]
    fn intersect() {
        use super::Intersection::*;

        let line_1 = Space::new(vector([4.046, 2.836]), 1.21);
        let line_2 = Space::new(vector([10.115, 7.09]), 3.025);
        assert_eq!(line_1.intersect(&line_2), Infinity);

        let line_1 = Space::new(vector([7.204, 3.182]), 8.68);
        let line_2 = Space::new(vector([8.172, 4.114]), 9.883);
        assert_eq!(
            line_1.intersect(&line_2).unwrap().round(3),
            vector([1.173, 0.073])
        );

        let line_1 = Space::new(vector([1.182, 5.562]), 6.744);
        let line_2 = Space::new(vector([1.773, 8.343]), 9.525);
        assert_eq!(line_1.intersect(&line_2), None);
    }
}
