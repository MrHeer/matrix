mod ops;

#[derive(Debug)]
pub struct Vector<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Vector<T, N> {
    pub fn size(&self) -> usize {
        return N;
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn size() {
        let v = Vector([2, 3, 3]);
        assert_eq!(v.size(), 3);

        let v = Vector::<i32, 0>([]);
        assert_eq!(v.size(), 0);
    }
}
