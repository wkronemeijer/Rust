use std::collections::HashMap;

use super::chunk::CHUNK_DIM;
use super::chunk::Chunk;
use super::chunk::ChunkToTileIndex;
use super::tile::Tile;
use super::traits::DeltaTime;

/////////////////
// Chunk index //
/////////////////

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldToChunkIndex {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

#[expect(unused, reason = "just interesting")]
const MAX_WORLD_DIM: usize = u16::MAX as usize * CHUNK_DIM;

impl WorldToChunkIndex {
    pub fn new(x: u16, y: u16, z: u16) -> Self { WorldToChunkIndex { x, y, z } }
}

////////////////
// World Size //
////////////////

#[derive(Debug, Clone, Copy)]
pub struct WorldSize {
    /// Maximum X coordinate.
    pub max_x: u16,
    /// Maximum Y coordinate.
    pub max_y: u16,
    /// Maximum Z coordinate.
    pub max_z: u16,
}

struct WorldSizeIter {
    size: WorldSize,
    x: u16,
    y: u16,
    z: u16,
    done: bool,
}

impl WorldSizeIter {
    pub fn new(size: WorldSize) -> Self {
        WorldSizeIter { size, x: 0, y: 0, z: 0, done: false }
    }

    fn index(&mut self) -> WorldToChunkIndex {
        WorldToChunkIndex { x: self.x, y: self.y, z: self.z }
    }

    fn advance(&mut self) {
        let WorldSize { max_x, max_y, max_z } = self.size;

        self.x += 1;
        if self.x >= max_x {
            self.x = 0;
            self.y += 1;
            if self.y >= max_y {
                self.y = 0;
                self.z += 1;
                if self.z >= max_z {
                    self.z = 0;
                    self.done = true;
                }
            }
        }
    }
}

impl Iterator for WorldSizeIter {
    type Item = WorldToChunkIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.done {
            let result = self.index();
            debug_assert!(self.size.contains(result));
            self.advance();
            Some(result)
        } else {
            None
        }
    }
}

impl WorldSize {
    pub fn new(x_size: u16, y_size: u16, z_size: u16) -> Self {
        WorldSize { max_x: x_size, max_y: y_size, max_z: z_size }
    }

    // TODO: Have an iterable version of this?
    pub fn span<F: FnMut(WorldToChunkIndex)>(self, mut body: F) {
        for z in 0..self.max_z {
            for y in 0..self.max_y {
                for x in 0..self.max_x {
                    body(WorldToChunkIndex { x, y, z })
                }
            }
        }
    }

    pub fn contains(self, p: WorldToChunkIndex) -> bool {
        p.x < self.max_x && p.y < self.max_y && p.z < self.max_z
    }

    pub fn iter(self) -> impl Iterator<Item = WorldToChunkIndex> {
        WorldSizeIter::new(self)
    }

    pub fn spread(self, p: WorldToChunkIndex) -> Option<usize> {
        if self.contains(p) {
            // width isn't used
            let step_y = usize::from(self.max_y);
            let step_z = usize::from(self.max_z);

            let x = usize::from(p.x);
            let y = step_y * usize::from(p.y);
            let z = step_z * step_y * usize::from(p.z);

            Some(x + y + z)
        } else {
            None
        }
    }
}

///////////
// World //
///////////

#[derive(Debug)]
pub struct World {
    // One chunk for now
    pub size: WorldSize,

    pub(crate) chunks: HashMap<WorldToChunkIndex, Chunk>,
}

fn cool_pattern(pos: ChunkToTileIndex) -> bool { (pos.x + 3 * pos.y) % 5 != 0 }

impl World {
    pub fn new() -> Self {
        let mut world =
            World { size: WorldSize::new(4, 4, 4), chunks: HashMap::new() };

        // Populate chunks
        // Maybe moved to a seperate function? and called seperately?
        world.size.span(|chunk_idx| {
            world.get_mut(chunk_idx).map(|chunk| {
                chunk.for_each_tile_mut(|pos, tile| match pos.z {
                    0..8 => *tile = Tile::Stone,
                    8..11 => *tile = Tile::Dirt,
                    11 => *tile = Tile::Grass,
                    12 if cool_pattern(pos) => *tile = Tile::Grass,
                    _ => {}
                });
            });
        });

        world
    }

    pub fn get(&self, index: WorldToChunkIndex) -> Option<&Chunk> {
        self.chunks.get(&index)
    }

    pub fn get_mut(&mut self, index: WorldToChunkIndex) -> Option<&mut Chunk> {
        if self.size.contains(index) {
            self.chunks.insert(index, Chunk::new());
            self.chunks.get_mut(&index)
        } else {
            None
        }
    }

    pub fn update(&mut self, _: DeltaTime) {}

    pub fn tick(&mut self) {}
}
