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
pub enum Facing {
    Up,
    North,
    East,
    South,
    West,
    Down,
}

impl Facing {
    pub fn unit_vector(&self) -> vec3 {
        match self {
            Self::Up => vec3::new(0.0, 1.0, 0.0),
            Self::Down => vec3::new(0.0, -1.0, 0.0),
            Self::South => vec3::new(0.0, 0.0, 1.0),
            Self::North => vec3::new(0.0, 0.0, -1.0),
            Self::East => vec3::new(1.0, 0.0, 0.0),
            Self::West => vec3::new(-1.0, 0.0, 0.0),
        }
    }

    pub fn flip(&self) -> Facing {
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
