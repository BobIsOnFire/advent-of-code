pub struct Groups<T: Iterator, const N: usize> {
    iter: T,
}

impl<T: Iterator, const N: usize> Iterator for Groups<T, N> {
    type Item = [T::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        (0..N)
            .map(|_| self.iter.next())
            .collect::<Option<Vec<T::Item>>>()
            .map(|v| {
                v.try_into()
                    .unwrap_or_else(|_| panic!("Wow, how did this happen?"))
            })
    }
}

impl<T: Iterator, const N: usize> From<T> for Groups<T, N> {
    fn from(iter: T) -> Self {
        Self { iter }
    }
}
