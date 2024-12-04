const RECURSION_LIMIT: usize = 4096;

pub struct Recursive<I1, I, F>
where
    I1: Iterator,
    I: Iterator<Item = I1::Item>,
    F: Fn(&I1::Item) -> Option<I>,
{
    initial_iter: I1,
    iters: Vec<I>,
    func: F,
}

impl<I1, I, F> Recursive<I1, I, F>
where
    I1: Iterator,
    I: Iterator<Item = I1::Item>,
    F: Fn(&I1::Item) -> Option<I>,
{
    pub const fn new(iter: I1, func: F) -> Self {
        Self {
            initial_iter: iter,
            iters: vec![],
            func,
        }
    }
}

impl<I1, I, F> Iterator for Recursive<I1, I, F>
where
    I1: Iterator,
    I: Iterator<Item = I1::Item>,
    F: Fn(&I1::Item) -> Option<I>,
{
    type Item = (usize, I1::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let next = loop {
            if self.iters.is_empty() {
                break self.initial_iter.next()?;
            }

            let last_iter = self
                .iters
                .last_mut()
                .expect("Already checked that self.iters is not empty");
            match last_iter.next() {
                Some(file) => break file,
                None => {
                    // Directory is exhausted, return to parent
                    let _ = self
                        .iters
                        .pop()
                        .expect("Already checked that self.iters is not empty");
                }
            }
        };

        let depth = self.iters.len();

        if let Some(it) = (self.func)(&next) {
            assert!(
                self.iters.len() < RECURSION_LIMIT,
                "Recursion limit reached"
            );
            self.iters.push(it);
        }

        Some((depth, next))
    }
}
