//! Exports standard library-like items.

pub mod memory_usage;

use winit::dpi::PhysicalSize;

use crate::ivec3;

//////////////////////////////
// Spreading 2D index to 1D //
//////////////////////////////

pub fn spread<const N: usize>(ivec3 { x, y, z }: ivec3) -> Option<usize> {
    match (usize::try_from(x), usize::try_from(y), usize::try_from(z)) {
        (Ok(x), Ok(y), Ok(z)) if x < N && y < N && z < N => {
            // ...this looks just like changing the base of a number
            Some(x + N * y + N * N * z)
        }
        _ => None,
    }
}

////////////////////////////
// Aspect ratio extension //
////////////////////////////

pub trait AspectRatioExt {
    /// Returns the (width &div; height) aspect ratio.
    fn aspect_ratio(&self) -> f32;
}

impl AspectRatioExt for PhysicalSize<u32> {
    fn aspect_ratio(&self) -> f32 {
        let width = self.width as f32;
        let height = self.height as f32;
        width / height
    }
}

///////////////
// Bit logic //
///////////////

pub const fn bits_needed<const DIM: usize>() -> u32 {
    usize::BITS - (DIM - 1).leading_zeros()
}
