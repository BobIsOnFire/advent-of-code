use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatrixIndex {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct VecMatrix<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> VecMatrix<T> {
    pub fn with_data(data: Vec<T>, width: usize) -> Self {
        Self { data, width }
    }

    pub fn new(width: usize) -> Self {
        Self::with_data(vec![], width)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.len() / self.width()
    }

    pub fn push(&mut self, elem: T) {
        self.data.push(elem)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, idx: MatrixIndex) -> Option<&T> {
        self.get_flat_idx(idx)
            .and_then(|data_idx| self.data.get(data_idx))
    }

    pub fn get_mut(&mut self, idx: MatrixIndex) -> Option<&mut T> {
        self.get_flat_idx(idx)
            .and_then(|data_idx| self.data.get_mut(data_idx))
    }

    pub fn left_idx(&self, idx: MatrixIndex) -> Option<MatrixIndex> {
        let MatrixIndex { row, col } = idx;
        if col == 0 {
            None
        } else {
            Some(MatrixIndex { row, col: col - 1 })
        }
    }

    pub fn right_idx(&self, idx: MatrixIndex) -> Option<MatrixIndex> {
        let MatrixIndex { row, col } = idx;
        if col >= self.width() - 1 {
            None
        } else {
            Some(MatrixIndex { row, col: col + 1 })
        }
    }

    pub fn up_idx(&self, idx: MatrixIndex) -> Option<MatrixIndex> {
        let MatrixIndex { row, col } = idx;
        if row == 0 {
            None
        } else {
            Some(MatrixIndex { row: row - 1, col })
        }
    }

    pub fn down_idx(&self, idx: MatrixIndex) -> Option<MatrixIndex> {
        let MatrixIndex { row, col } = idx;
        if row >= self.height() - 1 {
            None
        } else {
            Some(MatrixIndex { row: row + 1, col })
        }
    }

    pub fn iter_enumerate(&self) -> impl Iterator<Item = (MatrixIndex, &T)> {
        self.iter()
            .enumerate()
            .map(|(idx, item)| (self.get_matrix_idx(idx), item))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    fn get_flat_idx(&self, idx: MatrixIndex) -> Option<usize> {
        if idx.col >= self.width {
            None
        } else {
            Some(idx.row * self.width + idx.col)
        }
    }

    fn get_matrix_idx(&self, data_idx: usize) -> MatrixIndex {
        MatrixIndex {
            row: data_idx / self.width,
            col: data_idx % self.width,
        }
    }
}

impl<T> Index<MatrixIndex> for VecMatrix<T> {
    type Output = T;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<MatrixIndex> for VecMatrix<T> {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T> Extend<T> for VecMatrix<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter)
    }
}

impl<T> IntoIterator for VecMatrix<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a VecMatrix<T> {
    type Item = &'a T;
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut VecMatrix<T> {
    type Item = &'a mut T;
    type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}
