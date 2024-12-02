use crate::ivec3;

pub fn spread<const DIM: usize>(ivec3 { x, y, z }: ivec3) -> Option<usize> {
    match (usize::try_from(x), usize::try_from(y), usize::try_from(z)) {
        (Ok(x), Ok(y), Ok(z)) if x < DIM && y < DIM && z < DIM => {
            // ...this looks just like changing the base of a number
            Some(x + DIM * y + DIM * DIM * z)
        }
        _ => None,
    }
}
