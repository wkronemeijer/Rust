use std::ops::Index;
use std::ops::IndexMut;

use super::tile::Tile;
use crate::core::spread;
use crate::ivec3;

/// The length in one dimension of a 3D chunk.
pub const CHUNK_DIM: usize = 16;
/// The volume of a 3D chunk.
pub const CHUNK_VOLUME: usize = CHUNK_DIM * CHUNK_DIM * CHUNK_DIM;
/// Converts a 3D chunk index into a linear index.
const CHUNK_SPREAD: fn(index: ivec3) -> Option<usize> = spread::<CHUNK_DIM>;

/// A cube containing [CHUNK_VOLUME] tiles.
#[derive(Debug, Clone)]
pub struct Chunk {
    /// Invariant: contains exactly CHUNK_VOLUME elements
    tiles: Box<[Tile; CHUNK_VOLUME]>,
}

pub type ChunkIndex = ivec3;

impl Chunk {
    pub fn new() -> Self {
        // 4kb on the stack feels like a lot
        // I wonder if it'll become a problem
        Chunk { tiles: Box::new([const { Tile::Air }; CHUNK_VOLUME]) }
    }

    pub fn tiles(&self) -> &[Tile; CHUNK_VOLUME] { &*self.tiles }
    pub fn tiles_mut(&mut self) -> &mut [Tile; CHUNK_VOLUME] {
        &mut *self.tiles
    }

    pub fn get(&self, pos: ChunkIndex) -> Option<&Tile> {
        self.tiles.get(CHUNK_SPREAD(pos)?)
    }

    pub fn get_mut(&mut self, pos: ChunkIndex) -> Option<&mut Tile> {
        self.tiles.get_mut(CHUNK_SPREAD(pos)?)
    }
}

impl Index<ChunkIndex> for Chunk {
    type Output = Tile;

    fn index(&self, index: ChunkIndex) -> &Self::Output {
        &self.tiles[CHUNK_SPREAD(index).expect("chunk index out of bounds")]
    }
}

impl IndexMut<ChunkIndex> for Chunk {
    fn index_mut(&mut self, index: ChunkIndex) -> &mut Self::Output {
        &mut self.tiles[CHUNK_SPREAD(index).expect("chunk index out of bounds")]
    }
}

impl Chunk {
    #[inline]
    pub fn for_each_position<F: FnMut(ChunkIndex)>(mut op: F) {
        for z in 0..CHUNK_DIM {
            for y in 0..CHUNK_DIM {
                for x in 0..CHUNK_DIM {
                    op(ivec3(x as i32, y as i32, z as i32))
                }
            }
        }
    }

    pub fn for_each_tile<F: FnMut(ChunkIndex, &Tile)>(&self, mut func: F) {
        Self::for_each_position(|pos| func(pos, &self[pos]))
    }

    pub fn for_each_tile_mut<F: FnMut(ChunkIndex, &mut Tile)>(
        &mut self,
        mut func: F,
    ) {
        Self::for_each_position(|pos| func(pos, &mut self[pos]))
    }
}
