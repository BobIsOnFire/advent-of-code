mod end_on_err;
mod groups;
mod recursive;
mod windows_cycle;

pub use end_on_err::EndOnErr;
pub use groups::Groups;
pub use recursive::Recursive;
pub use windows_cycle::WindowsCycle;

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

pub trait ArrayIterators<T, const N: usize> {
    fn windows_cycle(&self) -> WindowsCycle<T, N>;
}

impl<T, const N: usize> ArrayIterators<T, N> for [T; N] {
    fn windows_cycle(&self) -> WindowsCycle<T, N> {
        WindowsCycle::new(self)
    }
}
