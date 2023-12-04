pub struct WindowsCycle<'a, T, const N: usize> {
    data: &'a [T; N],
    pos: usize,
}

impl<'a, T, const N: usize> WindowsCycle<'a, T, N> {
    pub fn new(data: &'a [T; N]) -> Self {
        Self { data, pos: 0 }
    }
}

impl<'a, T, const N: usize> Iterator for WindowsCycle<'a, T, N> {
    type Item = [&'a T; N];

    fn next(&mut self) -> Option<Self::Item> {
        let old_pos = self.pos;
        self.pos = (self.pos + 1) % N;

        let mut indices = [0; N];
        for (i, val) in indices.iter_mut().enumerate() {
            *val = i;
        }

        Some(indices.map(|idx| &self.data[(old_pos + idx) % N]))
    }
}
