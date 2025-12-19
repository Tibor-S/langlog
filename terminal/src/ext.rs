use std::{isize, marker::PhantomData, ops::Range};

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
