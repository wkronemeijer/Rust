// (0, 0, 0) is
// positive coords is the world
// negative coords is the tiled world

pub mod chunk;
pub mod tile;

use chunk::Chunk;
use tile::Tile;

use crate::ivec3;

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
        if let Some(tile_ref) = world.chunk.get_mut(ivec3(7, 7, 7)) {
            println!("setting stone");
            *tile_ref = Tile::Stone;
        }
        if let Some(tile_ref) = world.chunk.get_mut(ivec3(8, 7, 7)) {
            println!("setting grass");
            *tile_ref = Tile::Grass;
        }
        if let Some(tile_ref) = world.chunk.get_mut(ivec3(9, 7, 7)) {
            println!("setting stone");
            *tile_ref = Tile::Stone;
        }
        if let Some(tile_ref) = world.chunk.get_mut(ivec3(9, 7, 8)) {
            println!("setting grass");
            *tile_ref = Tile::Grass;
        }
        world
    }
}
