use crate::{vector, Vector};

impl<const DIM: usize> IntoIterator for Vector<DIM> {
    type Item = f64;

    type IntoIter = std::array::IntoIter<Self::Item, DIM>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const DIM: usize> FromIterator<f64> for Vector<DIM> {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        let mut arr = [0.; DIM];
        for (index, item) in iter.into_iter().enumerate() {
            arr[index] = item;
        }
        vector(arr)
    }
}

#[cfg(test)]
mod tests {
    use crate::{vector, Vector};

    #[test]
    fn into_iter() {
        let v = vector([2., 3., 5.]);
        let iter = v.into_iter();
        let mut vec = vec![];
        for item in iter {
            vec.push(item);
        }
        assert_eq!(vec, vec![2., 3., 5.]);
    }
    #[test]
    fn from_iter() {
        let v = vector([2., 3., 5.])
            .into_iter()
            .map(|x| x * 2.)
            .collect::<Vector<3>>();
        assert_eq!(v, vector([4., 6., 10.]));
    }
}
