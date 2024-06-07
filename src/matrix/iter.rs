use crate::{vector, Matrix, Vector};

impl<const ROW: usize, const COL: usize> IntoIterator for Matrix<ROW, COL> {
    type Item = Vector<COL>;

    type IntoIter = std::array::IntoIter<Self::Item, ROW>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const ROW: usize, const COL: usize> FromIterator<Vector<COL>> for Matrix<ROW, COL> {
    fn from_iter<T: IntoIterator<Item = Vector<COL>>>(iter: T) -> Self {
        let mut vectors = [vector([0.; COL]); ROW];
        for (index, item) in iter.into_iter().enumerate() {
            vectors[index] = item;
        }
        Self(vectors)
    }
}

#[cfg(test)]
mod tests {
    use crate::{matrix, Matrix};

    #[test]
    fn into_iter() {
        let v = matrix([[2., 3., 5.], [7., 2., 1.]]);
        let iter = v.into_iter();
        let mut vec = vec![];
        for vector in iter {
            for item in vector {
                vec.push(item);
            }
        }
        assert_eq!(vec, vec![2., 3., 5., 7., 2., 1.]);
    }

    #[test]
    fn from_iter() {
        let v = matrix([[2., 3., 5.], [7., 2., 1.]])
            .into_iter()
            .map(|x| x * 2.)
            .collect::<Matrix<2, 3>>();
        assert_eq!(v, matrix([[4., 6., 10.], [14., 4., 2.]]));
    }
}
