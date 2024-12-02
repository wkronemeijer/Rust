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

// UPPER_SNAKE makes it look like a macro lol

#[derive(Debug, Clone)]
pub struct Chunk {
    /// Invariant: contains exactly CHUNK_VOLUME elements
    tiles: Box<[Tile; CHUNK_VOLUME]>,
}

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

    pub fn get(&self, pos: ivec3) -> Option<&Tile> {
        self.tiles.get(CHUNK_SPREAD(pos)?)
    }

    pub fn get_mut(&mut self, pos: ivec3) -> Option<&mut Tile> {
        self.tiles.get_mut(CHUNK_SPREAD(pos)?)
    }
}

impl Index<ivec3> for Chunk {
    type Output = Tile;

    fn index(&self, index: ivec3) -> &Self::Output {
        &self.tiles[CHUNK_SPREAD(index).expect("chunk index out of bounds")]
    }
}

impl IndexMut<ivec3> for Chunk {
    fn index_mut(&mut self, index: ivec3) -> &mut Self::Output {
        &mut self.tiles[CHUNK_SPREAD(index).expect("chunk index out of bounds")]
    }
}

fn pos_from_usize(x: usize, y: usize, z: usize) -> ivec3 {
    ivec3(x as i32, y as i32, z as i32)
}

impl Chunk {
    pub fn for_each_tile<F: FnMut(ivec3, &Tile)>(&self, mut op: F) {
        for z in 0..CHUNK_DIM {
            for y in 0..CHUNK_DIM {
                for x in 0..CHUNK_DIM {
                    let pos = pos_from_usize(x, y, z);
                    op(pos, &self[pos])
                }
            }
        }
    }

    pub fn for_each_tile_mut<F: FnMut(ivec3, &mut Tile)>(&mut self, mut op: F) {
        for z in 0..CHUNK_DIM {
            for y in 0..CHUNK_DIM {
                for x in 0..CHUNK_DIM {
                    let pos = pos_from_usize(x, y, z);
                    op(pos, &mut self[pos])
                }
            }
        }
    }
}
