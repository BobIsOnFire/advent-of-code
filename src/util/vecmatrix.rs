use std::ops::{Index, IndexMut};

#[derive(Debug)]
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

    pub fn get(&self, idx: (usize, usize)) -> Option<&T> {
        self.data.get(self.data_idx(idx)?)
    }

    pub fn get_mut(&mut self, idx: (usize, usize)) -> Option<&mut T> {
        let idx = self.data_idx(idx)?;
        self.data.get_mut(idx)
    }

    pub fn iter_enumerate(&self) -> VecMatrixIter<'_, T> {
        VecMatrixIter {
            mat: self,
            row: 0,
            col: 0,
        }
    }

    fn data_idx(&self, idx: (usize, usize)) -> Option<usize> {
        if idx.1 >= self.width {
            return None;
        }
        Some(idx.0 * self.width + idx.1)
    }
}

impl<T> Index<(usize, usize)> for VecMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for VecMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T> Extend<T> for VecMatrix<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter)
    }
}

pub struct VecMatrixIter<'a, T> {
    mat: &'a VecMatrix<T>,
    row: usize,
    col: usize,
}

impl<'a, T> Iterator for VecMatrixIter<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let idx = (self.row, self.col);

        self.col += 1;
        if self.col >= self.mat.width {
            self.row += 1;
            self.col = 0;
        }

        self.mat.get(idx).map(|res| (idx, res))
    }
}
