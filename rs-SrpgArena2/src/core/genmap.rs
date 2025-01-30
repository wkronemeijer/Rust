//! Implements a generational map.
//!
//! Check the [`genmap`](https://docs.rs/genmap/latest/genmap/index.html) crate
//! for a good explanation.

use std::marker::PhantomData;
use std::mem::replace;
use std::num::NonZero;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Gen {
    value: NonZero<u32>,
}

impl Gen {
    const ONE: Gen = Gen { value: NonZero::new(1).unwrap() };

    fn next(self) -> Option<Gen> {
        Some(Gen { value: self.value.checked_add(1)? })
    }

    fn bump(&mut self) { *self = self.next().unwrap_or(Self::ONE) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GenMapIndex {
    inner: u32,
}

impl From<GenMapIndex> for usize {
    fn from(value: GenMapIndex) -> Self { value.inner.try_into().unwrap() }
}

impl From<usize> for GenMapIndex {
    fn from(value: usize) -> Self {
        GenMapIndex { inner: value.try_into().unwrap() }
    }
}

/// A handle for a value in a generational map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Handle<T> {
    generation: Gen,
    index: GenMapIndex,
    item_type: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn coerce<U>(self) -> Handle<U> {
        let Handle { generation, index, .. } = self;
        Handle { generation, index, item_type: PhantomData }
    }
}

#[derive(Debug, Clone)]
struct Slot<T> {
    generation: Gen,
    value: Option<T>,
}

impl<T> Slot<T> {
    pub fn new() -> Self { Self { generation: Gen::ONE, value: None } }

    pub fn is_empty(&self) -> bool { self.value.is_none() }

    #[inline]
    pub fn get(&self) -> Option<&T> { self.value.as_ref() }

    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> { self.value.as_mut() }

    pub fn fill(&mut self, value: T) {
        debug_assert!(self.is_empty(), "slot must be empty");
        self.value = Some(value);
    }

    pub fn clear(&mut self) {
        if let Some(_) = replace(&mut self.value, None) {
            self.generation.bump();
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenMap<T> {
    slots: Vec<Slot<T>>,
}

/////////////////////////
// Internal operations //
/////////////////////////

impl<T> GenMap<T> {
    fn reuse_empty_index(&self) -> Option<usize> {
        for (index, slot) in self.slots.iter().enumerate() {
            if slot.is_empty() {
                return Some(index)
            }
        }
        None
    }

    fn create_new_index(&mut self) -> usize {
        let index = self.slots.len();
        self.slots.push(Slot::new());
        index
    }

    fn new_index(&mut self) -> usize {
        if let Some(index) = self.reuse_empty_index() {
            index
        } else {
            self.create_new_index()
        }
    }

    fn to_index(&self, handle: Handle<T>) -> Option<usize> {
        let index: usize = handle.index.into();
        let slot = self.slots.get(index)?;
        if slot.generation == handle.generation { Some(index) } else { None }
    }

    fn get_slot(&self, handle: Handle<T>) -> Option<&Slot<T>> {
        let index = self.to_index(handle)?;
        self.slots.get(index)
    }

    fn get_slot_mut(&mut self, hendle: Handle<T>) -> Option<&mut Slot<T>> {
        let index = self.to_index(hendle)?;
        self.slots.get_mut(index)
    }
}

////////////////////////
// External interface //
////////////////////////

impl<T> GenMap<T> {
    /// Creates a new generational map with no items.
    pub fn new() -> Self { GenMap { slots: Vec::new() } }

    /// Creates a new generational map with the specified capacity and no items.
    pub fn with_capacity(cap: usize) -> Self {
        GenMap { slots: Vec::with_capacity(cap) }
    }

    /// Adds a value to this map, returning a handle to that value.
    #[must_use]
    pub fn insert(&mut self, value: T) -> Handle<T> {
        let index = self.new_index();
        let slot = self.slots.get_mut(index).unwrap();
        let generation = slot.generation;
        let index = index.into();
        slot.fill(value);
        Handle { generation, index, item_type: PhantomData }
    }

    pub fn remove(&mut self, handle: Handle<T>) {
        if let Some(slot) = self.get_slot_mut(handle) {
            slot.clear()
        }
    }

    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.get_slot(handle)?.get()
    }

    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        self.get_slot_mut(handle)?.get_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.slots.iter().filter_map(|slot| slot.get())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.slots.iter_mut().filter_map(|slot| slot.get_mut())
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use crate::core::genmap::GenMap;

    #[test]
    fn basic_usage() {
        let mut map = GenMap::new();
        let yes = map.insert("yes");
        let no = map.insert("no");

        assert_eq!(map.iter().count(), 2);
        assert_eq!(map.get(yes), Some(&"yes"));
        assert_eq!(map.get(no), Some(&"no"));

        map.remove(yes);

        assert_eq!(map.get(yes), None);
        assert_eq!(map.get(no), Some(&"no"));

        let maybe = map.insert("maybe");

        assert_eq!(map.get(yes), None);
        assert_eq!(map.get(no), Some(&"no"));
        assert_eq!(map.get(maybe), Some(&"maybe"));
    }
}
