//! Sub-byte integer types like `u7`.
//! The `ux` crate has issues, so we implement this stuff ourselves.
//!
//! MIDI uses only u4 and u7, so the scope is constrained.

#![allow(non_camel_case_types)]

use std::error::Error;
use std::fmt;
use std::ops::Deref;

///////////////////////////
// Conversion error type //
///////////////////////////

#[derive(Debug, Clone)]
pub struct TryFromIntError;

impl fmt::Display for TryFromIntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to convert: input is out of output range")
    }
}

impl Error for TryFromIntError {}

//////////////////
// Define macro //
//////////////////

macro_rules! define_uint {
    ($name:ident, $bits:expr, $base:ty) => {
        #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $name {
            value: $base,
        }

        impl $name {
            pub const MIN: Self = Self { value: 0 };
            pub const MAX: Self = Self { value: (1 << $bits) - 1 };
            pub const BITS: u32 = $bits;

            pub const fn new(value: $base) -> Option<Self> {
                if Self::MIN.value <= value && value <= Self::MAX.value {
                    Some(Self { value })
                } else {
                    None
                }
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.value.fmt(f)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.value.fmt(f)
            }
        }

        impl Deref for $name {
            type Target = $base;

            fn deref(&self) -> &Self::Target { &self.value }
        }

        impl From<$name> for $base {
            fn from(derived: $name) -> $base { derived.value }
        }

        impl TryFrom<$base> for $name {
            type Error = TryFromIntError;

            fn try_from(base: $base) -> Result<$name, Self::Error> {
                Self::new(base).ok_or(TryFromIntError)
            }
        }
    };
}

////////////////////
// Actual defines //
////////////////////

define_uint!(u4, 4, u8);
define_uint!(u7, 7, u8);
