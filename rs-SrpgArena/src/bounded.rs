// use num::{traits::SaturatingAdd, Bounded, ToPrimitive};
// type BoundInt = i64;
// trait BoundableInteger: SaturatingAdd + Ord + Bounded + TryFrom<BoundInt> {}

// impl<T> BoundableInteger for T where T: SaturatingAdd + Ord + Bounded + TryFrom<BoundInt> {}

// #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
// struct BoundedInteger<T: BoundableInteger, const lo: BoundInt, const hi: BoundInt>(T);

// impl<T: BoundableInteger, const lo: i64, const hi: i64> BoundedInteger<T, lo, hi> {
//     fn new(value: T) -> BoundedInteger<T, lo, hi> {
//         let val = lo.to_isize;
//         let min: T = (L::to).try_into().unwrap_or(T::min_value);
//         let max: T = (H::to).try_into().unwrap_or(T::max_value);
//         BoundedInteger(value.clamp(min, max))
//     }
// }

// type Percentage = BoundedInteger<u8, 0, 100>;

// fn test() {
//     let p = Percentage::new(2u8);
// }
