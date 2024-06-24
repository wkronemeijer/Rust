pub fn reset<T: Default>(var: &mut T) {
    *var = T::default();
}

pub fn slice_get_many_mut<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert_ne!(i, j, "cannot mutably borrow the same index");
    let i_ref: &mut T;
    let j_ref: &mut T;
    if i > j {
        (j_ref, i_ref) = slice_get_many_mut(slice, j, i);
    } else {
        let (lower, upper) = slice.split_at_mut(j);
        i_ref = &mut lower[i];
        j_ref = &mut upper[0];
    }
    (i_ref, j_ref)
}
