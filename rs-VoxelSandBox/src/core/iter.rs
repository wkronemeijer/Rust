//! Iteration-related items.

use std::ops::AddAssign;

use num_traits::ConstOne;
use num_traits::ConstZero;

pub struct IntegerTripleIter<T> {
    done: bool,
    x: T,
    max_x: T,
    y: T,
    max_y: T,
    z: T,
    max_z: T,
}

impl<T> IntegerTripleIter<T>
where
    T: Copy + ConstZero + ConstOne + AddAssign + Ord,
{
    pub fn new(max_x: T, max_y: T, max_z: T) -> Self {
        IntegerTripleIter {
            done: false,
            x: T::ZERO,
            max_x,
            y: T::ZERO,
            max_y,
            z: T::ZERO,
            max_z,
        }
    }

    fn current(&mut self) -> (T, T, T) { (self.x, self.y, self.z) }

    fn is_valid(&self, (x, y, z): (T, T, T)) -> bool {
        x < self.max_x && y < self.max_y && z < self.max_z
    }

    fn advance(&mut self) {
        let IntegerTripleIter { max_x, max_y, max_z, .. } = *self;
        self.x += T::ONE;
        if self.x >= max_x {
            self.x = T::ZERO;
            self.y += T::ONE;
            if self.y >= max_y {
                self.y = T::ZERO;
                self.z += T::ONE;
                if self.z >= max_z {
                    self.z = T::ZERO;
                    self.done = true;
                }
            }
        }
    }
}

impl<T> Iterator for IntegerTripleIter<T>
where
    T: Copy + ConstZero + ConstOne + AddAssign + Ord,
{
    type Item = (T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.done {
            let result = self.current();
            debug_assert!(self.is_valid(result));
            self.advance();
            Some(result)
        } else {
            None
        }
    }
}
