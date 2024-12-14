use self::Tile::*;

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
