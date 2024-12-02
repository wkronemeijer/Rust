use crate::ivec3;

pub fn spread(ivec3 { x, y, z }: ivec3, dim: usize) -> Option<usize> {
    match (usize::try_from(x), usize::try_from(y), usize::try_from(z)) {
        (Ok(x), Ok(y), Ok(z)) if x < dim && y < dim && z < dim => {
            Some(x + dim * y + dim * dim * z)
        }
        _ => None,
    }
}

pub fn spread_indices(dim: usize) -> impl Iterator<Item = ivec3> {
    (0..dim)
        .flat_map(move |z| (0..dim).map(move |y| ((z, y))))
        .flat_map(move |(z, y)| (0..dim).map(move |x| ((z, y, x))))
        .map(|(z, y, x)| {
            let x = i32::try_from(x).unwrap();
            let y = i32::try_from(y).unwrap();
            let z = i32::try_from(z).unwrap();
            ivec3(x, y, z)
        })
}
