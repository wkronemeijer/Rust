// (0, 0, 0) is
// positive coords is the world
// negative coords is the tiled world

use crate::vec3;

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

//////////
// Tile //
//////////

#[derive(Debug, Clone, Default)]
pub enum Tile {
    // Things to ponder:
    // 1. How to implement TileEntities?
    // 2. How to attach data like growth stage or facing?
    #[default]
    Air,

    Stone,
    Dirt,
    Grass,
    // Log(Facing),

    // ShortGrass(GrowthStage),
}

impl Tile {
    pub const fn new() -> Self { Self::Air }

    pub fn is_visible(&self) -> bool {
        match self {
            Self::Air => false,
            _ => true,
        }
    }

    pub fn is_solid(&self) -> bool {
        match self {
            Self::Air => false,
            _ => true,
        }
    }
}

///////////
// Chunk //
///////////

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

#[derive(Clone)]
pub struct Chunk {
    /// Invariant: contains exactly CHUNK_VOLUME elements
    data: Box<[Tile]>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { data: vec![Tile::new(); CHUNK_VOLUME].into_boxed_slice() }
    }

    pub fn tiles(&self) -> &[Tile] { &self.data }
}

// VerticalChunk //

pub const VERTICAL_CHUNK_COUNT: usize = 32;

pub const WORLD_HEIGHT: usize = VERTICAL_CHUNK_COUNT * CHUNK_SIZE;
pub const WORLD_HEIGHT_METERS: usize = WORLD_HEIGHT / 3;

#[derive(Clone)]
pub struct VerticalChunk {
    data: Box<[Chunk]>,
}

impl VerticalChunk {
    pub fn new() -> Self {
        Self { data: vec![Chunk::new(); CHUNK_VOLUME].into_boxed_slice() }
    }
}

///////////
// World //
///////////

pub struct World {}
