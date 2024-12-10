use super::chunk::Chunk;
use super::chunk::ChunkIndex;
use super::tile::Tile;

#[derive(Debug)]
pub struct World {
    // One chunk for now
    pub(crate) chunk: Chunk,
}

fn cool_pattern(pos: ChunkIndex) -> bool { (pos.x + 3 * pos.y) % 5 != 0 }

impl World {
    pub fn new() -> Self {
        let mut world = World { chunk: Chunk::new() };

        world.chunk.for_each_tile_mut(|pos, tile| match pos.z {
            0..8 => *tile = Tile::Stone,
            8..11 => *tile = Tile::Dirt,
            11 => *tile = Tile::Grass,
            12 if cool_pattern(pos) => *tile = Tile::Grass,
            _ => {}
        });

        world
    }

    pub fn tick(&mut self) {}
}
