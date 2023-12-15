pub struct EndOnErr<R, E, T: Iterator<Item = Result<R, E>>> {
    error: Option<E>,
    iter: T,
}

impl<R, E, T: Iterator<Item = Result<R, E>>> EndOnErr<R, E, T> {
    pub fn into_err(self) -> Result<(), E> {
        self.error.map_or(Ok(()), Err)
    }
}

impl<R, E, T: Iterator<Item = Result<R, E>>> Iterator for EndOnErr<R, E, T> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error.is_some() {
            return None;
        }

        match self.iter.next() {
            Some(Ok(val)) => Some(val),
            Some(Err(err)) => {
                self.error = Some(err);
                None
            }
            None => None,
        }
    }
}

impl<R, E, T: Iterator<Item = Result<R, E>>> From<T> for EndOnErr<R, E, T> {
    fn from(iter: T) -> Self {
        Self { error: None, iter }
    }
}
