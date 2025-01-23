use std::mem::swap;

/// Resets a reference to its default value.
pub fn reset<T: Default>(var: &mut T) { *var = T::default(); }

/// Allows for mutable access to two elements at distinct indices at once.
pub fn slice_index_pair_checked<T>(
    slice: &mut [T],
    (mut i, mut j): (usize, usize),
) -> Option<(&mut T, &mut T)> {
    let len = slice.len();
    if i == j {
        return None;
    }
    if i > j {
        swap(&mut j, &mut i);
    }
    if j >= len {
        return None;
    }
    // `0 <= i < j < len` now holds
    let (lower, upper) = slice.split_at_mut_checked(j)?;
    let item_i = lower.get_mut(i)?;
    let item_j = upper.get_mut(0)?;
    Some((item_i, item_j))
}