use super::chunk::Chunk;
use super::chunk::ChunkIndex;
use super::tile::Tile;
use super::traits::DeltaTime;
use crate::core::memory_usage::AllocatedSize;

#[derive(Debug)]
pub struct World {
    // One chunk for now
    pub(crate) chunk: Chunk,
}

fn cool_pattern(pos: ChunkIndex) -> bool { (pos.x + 3 * pos.y) % 5 != 0 }

// pub struct ChunkIndex(usize, usize, usize);
// Given XyzIndex
// is xyz the thing you are indexing into, or what you get out?

// TO BE CONTINUED

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChunkHandle {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

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

    pub fn update(&mut self, _: DeltaTime) {}

    pub fn tick(&mut self) {}
}

impl AllocatedSize for World {
    fn allocated_size(&self) -> usize { self.chunk.retained_size() }
}
