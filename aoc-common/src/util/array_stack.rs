#[derive(Clone)]
pub struct ArrayStack<T, const N: usize> {
    array: [Option<T>; N],
    head: usize,
}

impl<T: std::fmt::Debug + Copy, const N: usize> std::fmt::Debug for ArrayStack<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = vec![];
        for el in self.array.iter() {
            if el.is_none() {
                break;
            }

            res.push(el.unwrap());
        }
        write!(f, "{:?}", res)
    }
}

impl<T, const N: usize> Default for ArrayStack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> ArrayStack<T, N> {
    pub fn new() -> Self {
        Self {
            array: [(); N].map(|_| None),
            head: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.head
    }

    pub fn push(&mut self, elem: T) {
        if self.head == N {
            panic!("ArrayStack limit exceeded");
        }

        self.array[self.head] = Some(elem);
        self.head += 1;
    }

    pub fn top(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.array[self.head - 1].as_ref()
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.head -= 1;
            self.array[self.head].take()
        }
    }

    pub fn reverse(&mut self) {
        self.array[0..self.head].reverse()
    }
}
