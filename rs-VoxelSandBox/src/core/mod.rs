//! Exports standard library-like items.

pub mod fused_shader;
pub mod iter;
pub mod memory_usage;
pub mod spreading;

///////////////
// Bit logic //
///////////////

pub const fn bits_needed<const DIM: usize>() -> u32 {
    usize::BITS - (DIM - 1).leading_zeros()
}
