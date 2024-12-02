// (0, 0, 0) is
// positive coords is the world
// negative coords is the tiled world

pub mod chunk;
pub mod tile;

use chunk::Chunk;
use tile::Tile;

///////////
// World //
///////////

pub struct World {
    // One chunk for now
    pub(crate) chunk: Chunk,
}

impl World {
    pub fn new() -> Self {
        let mut world = World { chunk: Chunk::new() };

        world.chunk.for_each_tile_mut(|pos, tile| match pos.z {
            0..8 => {
                *tile = Tile::Stone;
            }
            8 => {
                *tile = Tile::Grass;
            }
            _ => {}
        });

        world
    }
}
