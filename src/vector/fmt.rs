use std::fmt::Display;

use crate::Vector;

impl<const N: usize> Display for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if N == 0 {
            return write!(f, "[]")
        }

        let self_arr = self.0;
        write!(f, "[")?;
        for i in 0..(N - 1) {
            write!(f, "{}, ", self_arr[i])?;
        }
        write!(f, "{}]", self_arr[N - 1])?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", Vector([])), "[]");
        assert_eq!(format!("{}", Vector([2., 1., 3.])), "[2, 1, 3]");
    }
}
