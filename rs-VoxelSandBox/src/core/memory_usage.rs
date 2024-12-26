//! Tools to report on memory usage.
/*
Contains many problems:
How do we prevent overlap?
How do we split used/reserved memory?




*/

/// Reports the lower bound of allocated size.
///
/// Based on https://stackoverflow.com/a/68255583
pub trait AllocatedSize {
    /// Returns the lower bound of the size of all owned heap allocations.
    fn allocated_size(&self) -> usize;

    /// The lower bound of bytes of memory this value "retains".
    fn memory_usage(&self) -> usize {
        size_of_val(self) + self.allocated_size()
    }
}

macro_rules! no_heap_impl {
    ($t:ty) => {
        impl AllocatedSize for $t {
            fn allocated_size(&self) -> usize { 0 }
        }
    };
}

no_heap_impl!(u8);
no_heap_impl!(u16);
no_heap_impl!(u32);
no_heap_impl!(u64);
no_heap_impl!(i8);
no_heap_impl!(i16);
no_heap_impl!(i32);
no_heap_impl!(i64);
no_heap_impl!(f32);
no_heap_impl!(f64);

impl<T: AllocatedSize + ?Sized> AllocatedSize for Box<T> {
    fn allocated_size(&self) -> usize { self.as_ref().memory_usage() }
}

impl<T: AllocatedSize> AllocatedSize for [T] {
    fn allocated_size(&self) -> usize { self.iter().map(T::memory_usage).sum() }
}

impl<T: AllocatedSize> AllocatedSize for Vec<T> {
    fn allocated_size(&self) -> usize { self.iter().map(T::memory_usage).sum() }
}
