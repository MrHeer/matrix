use std::fmt::Display;

use crate::Vector;

impl<const DIM: usize> Display for Vector<DIM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if DIM == 0 {
            return write!(f, "[]");
        }

        write!(f, "[")?;
        for index in 0..(DIM - 1) {
            self[index].fmt(f)?;
            write!(f, ", ")?;
        }
        self[DIM - 1].fmt(f)?;
        writeln!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use crate::vector;

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", vector([])), "[]");
        assert_eq!(format!("{}", vector([2., 1., 3.])), "[2, 1, 3]\n");
        assert_eq!(
            format!("{:.1}", vector([2.53, 1.7823, 3.161])),
            "[2.5, 1.8, 3.2]\n"
        );
    }
}
