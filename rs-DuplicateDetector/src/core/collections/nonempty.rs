use std::num::NonZero;
use std::ops::Deref;

/////////////
// Gen 2.0 //
/////////////
// TODO: Should I just use [`nunny`]?

/////////////////////
// Non-empty slice //
/////////////////////

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

//////////////////////
// Non-empty vector //
//////////////////////

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct NonEmptyVec<T> {
    inner: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    pub fn new(inner: Vec<T>) -> Option<Self> {
        if inner.len() > 0 { Some(NonEmptyVec { inner }) } else { None }
    }

    pub fn of(value: T) -> Self { NonEmptyVec { inner: vec![value] } }

    pub fn first(&self) -> &T { self.inner.first().unwrap() }

    pub fn last(&self) -> &T { self.inner.last().unwrap() }

    pub fn push(&mut self, value: T) { self.inner.push(value) }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target { &self.inner }
}
