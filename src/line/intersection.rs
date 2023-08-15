use crate::Vector;

#[derive(Debug, PartialEq)]
pub enum Intersection<const DIM: usize> {
    Some(Vector<DIM>),
    None,
    Infinity,
}

impl<const DIM: usize> Intersection<DIM> {
    pub fn unwrap(self) -> Vector<DIM> {
        use Intersection::*;
        match self {
            Some(val) => val,
            None => panic!("called `Intersection::unwrap()` on a `None` value"),
            Infinity => panic!("called `Intersection::unwrap()` on a `Infinity` value"),
        }
    }
}
