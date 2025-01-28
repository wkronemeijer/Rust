use std::num::NonZero;
use std::ops::Deref;

#[repr(transparent)]
pub struct NonEmptySlice<'a, T> {
    inner: &'a [T],
}

impl<'a, T> NonEmptySlice<'a, T> {
    pub fn new(inner: &'a [T]) -> Option<Self> {
        if inner.len() > 0 { Some(NonEmptySlice { inner }) } else { None }
    }

    pub fn len(&self) -> NonZero<usize> {
        NonZero::new(self.inner.len()).unwrap()
    }
}

impl<'a, T> Deref for NonEmptySlice<'a, T> {
    type Target = &'a [T];

    fn deref(&self) -> &Self::Target { &self.inner }
}
