use self::Tile::*;
use crate::core::memory_usage::AllocatedSize;
use crate::ivec2;

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

    pub fn tex_loc(&self) -> ivec2 {
        // NB: tile indices wrap around!
        match self {
            Air => ivec2(-1, 0),
            Stone => ivec2(-1, 1),
            Dirt => ivec2(-1, 3),
            Grass => ivec2(-1, 2),
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

impl AllocatedSize for Tile {
    fn allocated_size(&self) -> usize { 0 }
}
