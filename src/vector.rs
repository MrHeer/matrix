mod fmt;
mod ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Vector<T, N> {
    pub fn size(&self) -> usize {
        N
    }

    pub fn map<F, U>(self, f: F) -> Vector<U, N>
    where
        F: FnMut(T) -> U,
    {
        let result_arr = self.0.map(f);
        Vector(result_arr)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn size() {
        let v = Vector([2, 3, 3]);
        assert_eq!(v.size(), 3);

        let v = Vector::<i32, 0>([]);
        assert_eq!(v.size(), 0);
    }

    #[test]
    fn map() {
        let v = Vector([2, 3, 1]);
        let double = |x: i32| x * 2;

        assert_eq!(v.map(double), Vector([4, 6, 2]));
    }
}
