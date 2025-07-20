use std::ops::Mul;

use num_traits::ConstOne;
use num_traits::ConstZero;

#[derive(Clone, Copy)]
pub struct Ratio<T> {
    // Ideally, we have Ratio<T, U, V>
    // where numer: T, denom: U, and lhs.numer * rhs.denom : V
    // Then try_from back to T and U
    //
    pub numer: T,
    pub denom: T, // NonZero
}

impl<T: ConstOne> Ratio<T> {
    pub const fn new(numer: T, denom: T) -> Self { Ratio { numer, denom } }

    pub const ONE: Self = Ratio { numer: T::ONE, denom: T::ONE };
}

impl<T: ConstZero + ConstOne> Ratio<T> {
    pub const ZERO: Self = Ratio { numer: T::ZERO, denom: T::ONE };
}

impl<T: Mul<Output = T>> Mul for Ratio<T> {
    type Output = Self;

    fn mul(self, that: Self) -> Self::Output {
        Ratio { numer: self.numer * that.numer, denom: self.denom * that.denom }
    }
}
