//! A custom implementation of [`nunny`](https://crates.io/crates/nunny).

use std::num::NonZero;
use std::ops::Deref;

/////////////
// Gen 2.0 //
/////////////
// TODO: Should I just use [`nunny`]?

/////////////////////
// Non-empty slice //
/////////////////////

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
/// A slice that is statically known to be non-empty.
pub struct NonEmptySlice<'a, T> {
    inner: &'a [T],
}

impl<'a, T> NonEmptySlice<'a, T> {
    /// Refines a slice on being non-empty.
    pub fn new(inner: &'a [T]) -> Option<Self> {
        if inner.len() > 0 { Some(NonEmptySlice { inner }) } else { None }
    }

    /// Returns the length this slice.
    pub fn len(&self) -> NonZero<usize> {
        NonZero::new(self.inner.len()).unwrap()
    }

    /// Returns the first element of this slice.
    /// Unlike the basic slice, this method's return is not [`Option`].
    pub fn first(self) -> &'a T { self.inner.first().unwrap() }

    /// Returns the last element of this slice.
    /// Unlike the basic slice, this method's return is not [`Option`].
    pub fn last(self) -> &'a T { self.inner.last().unwrap() }
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
/// A vector statically known to be non-empty.
pub struct NonEmptyVec<T> {
    inner: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    /// Refines a vector to be non-empty.
    pub fn new(inner: Vec<T>) -> Option<Self> {
        if inner.len() > 0 { Some(NonEmptyVec { inner }) } else { None }
    }

    /// Creates a non-empty vector with 1 element.
    pub fn of(value: T) -> Self { NonEmptyVec { inner: vec![value] } }

    /// Access this vector as a slice.
    pub fn as_slice(&self) -> NonEmptySlice<T> {
        NonEmptySlice::new(&self.inner).unwrap()
    }

    /// Extends this vector with another element.
    pub fn push(&mut self, value: T) { self.inner.push(value) }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target { &self.inner }
}
