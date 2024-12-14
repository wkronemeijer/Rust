use crate::ivec3;
use crate::vec3;

// TODO: Should we use Zenith/Nadir for up and down?
/// A face of a axis-aligned cube.
#[derive(Debug, Clone, Copy)]
pub enum Face {
    /// Points toward +Z
    Up,
    /// Points toward +Y
    North,
    /// Points toward +X
    East,
    /// Points toward -Y
    South,
    /// Points toward -X
    West,
    /// Points toward -Z
    Down,
}

impl Face {
    pub const COUNT: usize = 6;

    pub const ALL: &[Face; Self::COUNT] = &[
        Self::Up,
        Self::North,
        Self::East,
        Self::South,
        Self::West,
        Self::Down,
    ];

    pub fn vec3(&self) -> vec3 {
        match self {
            Self::Up => vec3::Z,
            Self::Down => vec3::NEG_Z,
            Self::South => vec3::NEG_Y,
            Self::North => vec3::Y,
            Self::East => vec3::X,
            Self::West => vec3::NEG_X,
        }
    }

    pub fn ivec3(self) -> ivec3 {
        match self {
            Self::Up => ivec3::Z,
            Self::Down => ivec3::NEG_Z,
            Self::South => ivec3::NEG_Y,
            Self::North => ivec3::Y,
            Self::East => ivec3::X,
            Self::West => ivec3::NEG_X,
        }
    }

    pub fn flip(&self) -> Face {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}
