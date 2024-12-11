use crate::vec3;

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
    pub const ALL: &[Face; 6] = &[
        Self::Up,
        Self::Down,
        Self::South,
        Self::North,
        Self::East,
        Self::West,
    ];

    pub fn unit_vector(&self) -> vec3 {
        match self {
            Self::Up => vec3::Z,
            Self::Down => vec3::NEG_Z,
            Self::South => vec3::NEG_Y,
            Self::North => vec3::Y,
            Self::East => vec3::X,
            Self::West => vec3::NEG_X,
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
