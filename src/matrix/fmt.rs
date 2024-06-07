use std::fmt::Display;

use crate::Matrix;

impl<const ROW: usize, const COL: usize> Display for Matrix<ROW, COL> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if ROW == 0 {
            return write!(f, "[]");
        }

        for vector in self.into_iter() {
            vector.fmt(f)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", matrix::<0, 0>([])), "[]");
        assert_eq!(format!("{}", matrix([[2., 1., 3.]])), "[2, 1, 3]\n");
        assert_eq!(
            format!("{}", matrix([[2., 1., 3.], [5., 7., 8.]])),
            "[2, 1, 3]\n[5, 7, 8]\n"
        );
        assert_eq!(
            format!("{:.1}", matrix([[2.53, 1.7823, 3.161]])),
            "[2.5, 1.8, 3.2]\n"
        );
    }
}
