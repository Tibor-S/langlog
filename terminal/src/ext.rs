use std::{cmp::Ordering, isize, marker::PhantomData, ops::Range};

/// # Range with mid
/// ```
/// mid = 35
/// len = 7
/// start = mid - len / 2 - len&1 + 1 = 32
/// 32  33  34  35  36  37  38
///             ^^
/// len = 8
/// start = mid - len / 2 - len&1 + 1 = 32
/// 32  33  34  35  36  37  38  39
///             ^^
/// ```
pub fn range_with_mid(mid: isize, len: isize) -> Range<isize> {
    if len == 0 {
        return mid..mid;
    }
    let start = mid - (len / 2) - (len & 1) + 1;
    start..(start + len)
}

/// # Saturate Range <u16>
/// Returns a range `ret` where:
///  - `value.len() == ret.len()`
///  - `domain.start <= ret.start`
///  - `value.start <= domain.start --> domain.start == ret.start`
///  - `ret.end <= domain.end`
///  - `domain.end <= value.end --> domain.end == ret.end`
/// ## Panics
/// if `value.len() > domain.len()`
pub fn saturate_range(
    value: Range<isize>,
    domain: Range<usize>,
) -> Range<usize> {
    assert!(value.len() <= domain.len());
    if lt_iu(value.start, domain.start) {
        domain.start..(domain.start + value.len())
    } else if gt_iu(value.end, domain.end) {
        (domain.end - value.len())..(domain.end)
    } else {
        value.start as usize..value.end as usize
    }
}

pub fn lt_iu(lhs: isize, rhs: usize) -> bool {
    (isize::MAX as usize).cmp(&rhs).is_lt() || lhs < rhs as isize
}
pub fn gt_iu(lhs: isize, rhs: usize) -> bool {
    (isize::MAX as usize).cmp(&rhs).is_gt() && lhs > rhs as isize
}

/// Returns the first index `i` where `comp(value, &vec\[i\]) == Ordering::Less`
///
/// If no such index exists then `vec.len()` is returned
///
/// https://en.cppreference.com/w/cpp/algorithm/upper_bound.html
pub fn upper_bound<T, V, F>(vec: &Vec<T>, value: &V, comp: F) -> usize
where
    F: Fn(&V, &T) -> Ordering,
{
    let mut first = 0;
    let last = vec.len();
    let mut count = last - first;
    let mut i;
    let mut step: usize;

    while count > 0 {
        i = first;
        step = count / 2;
        i += step;

        if comp(value, &vec[i]) != Ordering::Less {
            i += 1;
            first = i;
            count -= step + 1;
        } else {
            count = step;
        }
    }
    return first;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fork<Tr, Fa, R>
where
    Tr: Iterator<Item = R>,
    Fa: Iterator<Item = R>,
{
    True(Tr, PhantomData<R>),
    False(Fa, PhantomData<R>),
}
impl<Tr, Fa, R> Fork<Tr, Fa, R>
where
    Tr: Iterator<Item = R>,
    Fa: Iterator<Item = R>,
{
    pub fn either<T, U>(check: bool, if_true: T, if_false: U) -> Self
    where
        T: IntoIterator<Item = R, IntoIter = Tr>,
        U: IntoIterator<Item = R, IntoIter = Fa>,
    {
        if check {
            Self::True(if_true.into_iter(), Default::default())
        } else {
            Self::False(if_false.into_iter(), Default::default())
        }
    }
}
impl<T, U, R> Iterator for Fork<T, U, R>
where
    T: Iterator<Item = R>,
    U: Iterator<Item = R>,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Fork::True(iter, _) => iter.next(),
            Fork::False(iter, _) => iter.next(),
        }
    }
}
pub trait IntoFork: Iterator {
    fn fork_if<T>(
        self,
        check: bool,
        if_true: T,
    ) -> Fork<T::IntoIter, Self, Self::Item>
    where
        T: IntoIterator<Item = Self::Item>,
        Self: Sized,
    {
        Fork::either(check, if_true, self)
    }
}
impl<I: Iterator> IntoFork for I {}

#[cfg(test)]
mod tests {
    use crate::ext::upper_bound;

    #[test]
    fn test_upper_bound() {
        //           0  1  2  3  4  5  6  7  8  9   10  11   12   13
        let v = vec![1, 2, 3, 3, 3, 3, 4, 5, 9, 10, 23, 543, 611];
        assert_eq!(
            upper_bound(&v, &0usize, |val: &usize, el: &usize| val.cmp(el)),
            0usize
        );
        assert_eq!(
            upper_bound(&v, &3usize, |val: &usize, el: &usize| val.cmp(el)),
            6usize
        );
        assert_eq!(
            upper_bound(&v, &10usize, |val: &usize, el: &usize| val.cmp(el)),
            10usize
        );
        assert_eq!(
            upper_bound(&v, &611usize, |val: &usize, el: &usize| val.cmp(el)),
            v.len()
        );
    }
}
pub fn call_nullary<T, F: Fn() -> T>(f: &F) -> T {
    f()
}
pub fn call_unary<T, U, F: Fn(T) -> U>(f: &F, arg: T) -> U {
    f(arg)
}
