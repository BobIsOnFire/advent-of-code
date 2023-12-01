mod end_on_err;
mod groups;
mod recursive;

pub use end_on_err::EndOnErr;
pub use groups::Groups;
pub use recursive::Recursive;

pub trait IteratorExtended: Iterator + Sized {
    fn groups<const N: usize>(self) -> Groups<Self, N> {
        self.into()
    }

    fn recursive<I, F>(self, func: F) -> Recursive<Self, I, F>
    where
        I: Iterator<Item = Self::Item>,
        F: Fn(&Self::Item) -> Option<I>,
    {
        Recursive::new(self, func)
    }
}

impl<T: Iterator> IteratorExtended for T {}

pub trait ResultIteratorExtended<R, E>: Iterator<Item = Result<R, E>> + Sized {
    fn end_on_error(self) -> EndOnErr<R, E, Self> {
        self.into()
    }
}

impl<R, E, T: Iterator<Item = Result<R, E>>> ResultIteratorExtended<R, E> for T {}
