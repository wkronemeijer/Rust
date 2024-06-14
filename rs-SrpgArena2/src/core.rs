use std::mem::swap;

pub fn reset<T: Default>(var: &mut T) {
    *var = T::default();
    // You can in fact do this
    // ...
}

pub fn sort_pair_asc<T: Ord>(lhs: &mut T, rhs: &mut T) {
    if lhs > rhs {
        swap(lhs, rhs);
    }
    debug_assert!(lhs <= rhs);
}

pub fn slice_get_many_mut<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert_ne!(i, j, "cannot mutably borrow the same index");
    if i > j {
        let (j_ref, i_ref) = slice_get_many_mut(slice, j, i);
        (i_ref, j_ref)
    } else {
        let (lower, upper) = slice.split_at_mut(j);
        let fst = &mut lower[i];
        let snd = &mut upper[0];
        (fst, snd)
    }
}
