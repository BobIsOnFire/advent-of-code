pub trait Bisect: PartialEq {
    fn next(&self) -> Self;
    fn between(&self, other: &Self) -> Self;
}

macro_rules! bisect_numeric_impl {
    ($type:ty) => {
        impl Bisect for $type {
            fn next(&self) -> Self {
                self + 1
            }

            fn between(&self, other: &Self) -> Self {
                let lo = self.min(other);
                let hi = self.max(other);
                lo + (hi - lo) / 2
            }
        }
    };
}

bisect_numeric_impl!(i8);
bisect_numeric_impl!(i16);
bisect_numeric_impl!(i32);
bisect_numeric_impl!(i64);
bisect_numeric_impl!(isize);
bisect_numeric_impl!(u8);
bisect_numeric_impl!(u16);
bisect_numeric_impl!(u32);
bisect_numeric_impl!(u64);
bisect_numeric_impl!(usize);

pub fn bisect<T: Bisect>(bad: T, good: T, is_good: impl Fn(&T) -> bool) -> T {
    assert!(is_good(&good), "Bad value submitted as 'good' arg for bisect");
    assert!(!is_good(&bad), "Good value submitted as 'bad' arg for bisect");

    let mut lo = bad;
    let mut hi = good;
    let mut mid = <T as Bisect>::between(&lo, &hi);
    while lo != hi {
        if is_good(&mid) {
            hi = mid;
        } else {
            lo = mid.next();
        }
        mid = <T as Bisect>::between(&lo, &hi);
    }

    mid
}
