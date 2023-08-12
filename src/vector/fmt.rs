use std::fmt::Display;

use crate::Vector;

impl<const DIM: usize> Display for Vector<DIM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if DIM == 0 {
            return write!(f, "[]");
        }

        let self_arr = self.0;
        write!(f, "[")?;
        for i in 0..(DIM - 1) {
            write!(f, "{}, ", self_arr[i])?;
        }
        write!(f, "{}]", self_arr[DIM - 1])?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::vector;

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", vector([])), "[]");
        assert_eq!(format!("{}", vector([2., 1., 3.])), "[2, 1, 3]");
    }
}
