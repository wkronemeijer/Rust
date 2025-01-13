//! Problem: flatten a N-dimensional index into a linear index (for e.g. arrays)
//! Solution: the items in this module

use std::iter::empty;

use crate::ivec2;
use crate::ivec3;
use crate::uvec2;
use crate::uvec3;

pub fn const_spread<const N: usize>(ivec3 { x, y, z }: ivec3) -> Option<usize> {
    match (usize::try_from(x), usize::try_from(y), usize::try_from(z)) {
        (Ok(x), Ok(y), Ok(z)) if x < N && y < N && z < N => {
            // ...this looks just like changing the base of a number
            Some(x + N * y + N * N * z)
        },
        _ => None,
    }
}

//////////////////////////////////////////
// Iterating integer points inside size //
//////////////////////////////////////////

fn every_vec1_u32(this: u32) -> impl Iterator<Item = i32> + Clone {
    let start = 0;
    let end = i32::try_from(this).unwrap_or(i32::MAX);
    start..end
}

// When can we have gen functions...
fn every_vec2_u32(this: uvec2) -> impl Iterator<Item = ivec2> {
    todo!();
    empty()
}

fn every_vec3_u32(this: uvec3) -> impl Iterator<Item = ivec3> {
    todo!();
    empty()
}

////////////////////
// SizeLike trait //
////////////////////

// What to call this...
// As in: a size of only integer dimensions

// Issues:
// panic_if_empty!
// 8th bit of unsigned types isn't actually usuable, also generates panics
// Advantage is that unsigned == size, signed == offset

// You /could/ use a wrapper struct
// But meeeeeeh
pub trait SizeLike
where
    Self: Copy, {
    type Offset: Copy;

    /// Returns whether this size is "empty", i.e. contains no points at all.
    fn is_empty(self) -> bool;

    /// Returns whether the offset is contained within this size.
    /// Negative values should always be rejected.
    fn contains(self, offset: Self::Offset) -> bool;

    /// Iterates over every contained offset.
    fn every_offset(self) -> impl Iterator<Item = Self::Offset>;

    /// Returns the offset if it is contained.
    /// Otherwise returns [`None`].
    fn checked_restrict(self, offset: Self::Offset) -> Option<Self::Offset> {
        if self.contains(offset) { Some(offset) } else { None }
    }

    /// Spreads an offset into a linear value.
    /// Should return [`None`] if the offset is not contained.
    fn checked_spread(self, offset: Self::Offset) -> Option<usize>;

    /// Restricts the offset to this size.
    /// Saturates at the boundaries.
    /// May panic if the size is empty.
    fn saturating_restrict(self, offset: Self::Offset) -> Self::Offset;

    /// Returns an index for a point in this size
    /// Saturates at the boundaries.
    /// May panic if the size is empty.
    fn saturating_spread(self, offset: Self::Offset) -> usize {
        self.checked_spread(self.saturating_restrict(offset))
            .expect("saturating_restrict return out of bounds")
    }

    /// Restricts the offset to this size
    /// Wraps at the boundaries.
    /// May panic if the size is empty.
    fn wrapping_restrict(self, offset: Self::Offset) -> Self::Offset;

    /// Returns an index for a point in this size.
    /// Wraps at the boundaries.
    /// May panic if the size is empty.
    fn wrapping_spread(self, offset: Self::Offset) -> usize {
        self.checked_spread(self.wrapping_restrict(offset))
            .expect("wrapping_restrict return out of bounds")
    }
}

////////////////////
// SizeLike impls //
////////////////////

macro_rules! panic_if_empty {
    ($e:expr) => {
        if $e.is_empty() {
            panic!("size is empty");
        }
    }
}

// There is no rem_unsigned method as far as I can see
// Not even under num_traits!
// Based on https://stackoverflow.com/a/74910522
fn i32_rem_unsigned(i: i32, n: u32) -> u32 {
    // n == 0 will get caught by % itself
    match u32::try_from(i) {
        Ok(i) => i % n,
        Err(_) => {
            let r = (!i as u32) % n;
            // i == -5, n == 3
            // !i == 4         (off by one, because of two's complement)
            // !i % n == 1     (off by one, again)
            n - r - 1
            // +1 would work, but then you need to % it again
        },
    }
}

impl SizeLike for u32 {
    type Offset = i32;

    fn is_empty(self) -> bool { self == 0 }

    fn contains(self, offset: Self::Offset) -> bool {
        match u32::try_from(offset) {
            Ok(offset) => offset < self,
            Err(_) => false,
        }
    }

    fn checked_spread(self, offset: Self::Offset) -> Option<usize> {
        if !self.contains(offset) {
            return None
        }
        usize::try_from(offset).ok()
    }

    fn saturating_restrict(self, offset: Self::Offset) -> Self::Offset {
        panic_if_empty!(self);
        let offset_u32 = u32::try_from(offset).unwrap_or(0);
        // 2 000 000 i32 ==> Ok(2 000 000 u32) ==> 2 000 000
        //         3 i32 ==> Ok(        3 u32) ==>         3
        //       -20 i32 ==> Err(_)            ==>         0
        let result = offset_u32.min(self);
        // offset_u32 <= i32::MAX, so always sound
        i32::try_from(result).unwrap()
    }

    fn wrapping_restrict(self, offset: Self::Offset) -> Self::Offset {
        panic_if_empty!(self);

        let result = i32_rem_unsigned(offset, self);
        // -1i32 % u32::MAX can create problems
        i32::try_from(result).expect("size too big")
    }

    #[inline]
    fn every_offset(self) -> impl Iterator<Item = Self::Offset> {
        every_vec1_u32(self)
    }
}

// Silly macro to prevent copy-pasting the wrong field
macro_rules! lift {
    ($lhs:expr, $method:ident, $rhs:expr, $field:ident) => {
        $lhs.$field.$method($rhs.$field)
    };
}

impl SizeLike for uvec2 {
    type Offset = ivec2;

    fn is_empty(self) -> bool { self.element_product() == 0 }

    fn contains(self, offset: Self::Offset) -> bool {
        lift!(self, contains, offset, x) && lift!(self, contains, offset, y)
    }

    fn checked_spread(self, offset: Self::Offset) -> Option<usize> {
        if !self.contains(offset) {
            return None
        }
        let x = lift!(self, checked_spread, offset, x)?;
        let y = lift!(self, checked_spread, offset, y)?;

        let x_stride = 1usize;
        let y_stride = usize::try_from(self.x).ok()?.checked_mul(x_stride)?;

        let x_offset = x.checked_mul(x_stride)?;
        let y_offset = y.checked_mul(y_stride)?;

        Some(x_offset.checked_add(y_offset)?)
    }

    fn saturating_restrict(self, offset: Self::Offset) -> Self::Offset {
        ivec2(
            lift!(self, saturating_restrict, offset, x),
            lift!(self, saturating_restrict, offset, y),
        )
    }

    fn wrapping_restrict(self, offset: Self::Offset) -> Self::Offset {
        ivec2(
            lift!(self, wrapping_restrict, offset, x),
            lift!(self, wrapping_restrict, offset, y),
        )
    }

    #[inline]
    fn every_offset(self) -> impl Iterator<Item = Self::Offset> {
        every_vec2_u32(self)
    }
}

impl SizeLike for uvec3 {
    type Offset = ivec3;

    fn is_empty(self) -> bool { self.element_product() == 0 }

    fn contains(self, offset: Self::Offset) -> bool {
        lift!(self, contains, offset, x) &&
            lift!(self, contains, offset, y) &&
            lift!(self, contains, offset, z)
    }

    fn checked_spread(self, offset: Self::Offset) -> Option<usize> {
        if !self.contains(offset) {
            return None
        }
        if !self.contains(offset) {
            return None
        }
        let x = lift!(self, checked_spread, offset, x)?;
        let y = lift!(self, checked_spread, offset, y)?;
        let z = lift!(self, checked_spread, offset, z)?;

        let x_stride = 1usize;
        let y_stride = usize::try_from(self.x).ok()?.checked_mul(x_stride)?;
        let z_stride = usize::try_from(self.y).ok()?.checked_mul(y_stride)?;

        let x_offset = x.checked_mul(x_stride)?;
        let y_offset = y.checked_mul(y_stride)?;
        let z_offset = z.checked_mul(z_stride)?;

        Some(x_offset.checked_add(y_offset)?.checked_add(z_offset)?)
    }

    fn saturating_restrict(self, offset: Self::Offset) -> Self::Offset {
        ivec3(
            lift!(self, saturating_restrict, offset, x),
            lift!(self, saturating_restrict, offset, y),
            lift!(self, saturating_restrict, offset, z),
        )
    }

    fn wrapping_restrict(self, offset: Self::Offset) -> Self::Offset {
        ivec3(
            lift!(self, wrapping_restrict, offset, x),
            lift!(self, wrapping_restrict, offset, y),
            lift!(self, wrapping_restrict, offset, z),
        )
    }

    #[inline]
    fn every_offset(self) -> impl Iterator<Item = Self::Offset> {
        every_vec3_u32(self)
    }
}
