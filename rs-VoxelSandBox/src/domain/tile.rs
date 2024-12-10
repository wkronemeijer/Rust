use self::Tile::*;
use crate::vec3;

//////////
// Tile //
//////////

#[derive(Debug, Clone, Default)]
pub enum Tile {
    #[default]
    Air,

    Stone,
    Dirt,
    Grass,
    // Log(Facing),
    // ShortGrass(GrowthStage),
}

impl Tile {
    pub const fn new() -> Self { Tile::Air }
}

// TODO: Pivot this so you have Tile => TileAttributes
// It's really a giant table, expression problem comes to mind
impl Tile {
    pub fn tex_index(&self) -> usize {
        match self {
            Air => 0,
            Stone => 1,
            Dirt => 3,
            Grass => 2,
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            Air => false,
            _ => true,
        }
    }

    pub fn is_solid(&self) -> bool {
        match self {
            Air => false,
            _ => true,
        }
    }
}

////////////
// Facing //
////////////

#[derive(Debug, Clone)]
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
