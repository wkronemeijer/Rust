use super::chunk::Chunk;
use super::tile::Tile;

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

    pub fn tick(&mut self) { println!("tick") }
}
