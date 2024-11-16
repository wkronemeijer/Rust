use std::mem::swap;

pub fn reset<T: Default>(var: &mut T) {
    *var = T::default();
}

pub fn slice_index_pair<T>(slice: &mut [T], (mut i, mut j): (usize, usize)) -> (&mut T, &mut T) {
    assert_ne!(i, j, "cannot mutably borrow the same index");
    let i_ref: &mut T;
    let j_ref: &mut T;
    if i > j {
        swap(&mut j, &mut i);
    }
    let (lower, upper) = slice.split_at_mut(j);
    i_ref = &mut lower[i];
    j_ref = &mut upper[0];
    (i_ref, j_ref)
}
