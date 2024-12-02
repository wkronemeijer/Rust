use std::ops::Index;
use std::ops::IndexMut;

use super::tile::Tile;
use crate::core::spread;
use crate::core::spread_indices;
use crate::ivec3;

/// The length in one dimension of a 3D chunk.
pub const CHUNK_LEN: usize = 16;
/// The volume of a 3D chunk.
pub const CHUNK_VOLUME: usize = CHUNK_LEN * CHUNK_LEN * CHUNK_LEN;

fn chunk_spread(index: ivec3) -> Option<usize> { spread(index, CHUNK_LEN) }
fn chunk_indices() -> impl Iterator<Item = ivec3> { spread_indices(CHUNK_LEN) }

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
        self.tiles.get(chunk_spread(pos)?)
    }

    pub fn get_mut(&mut self, pos: ivec3) -> Option<&mut Tile> {
        self.tiles.get_mut(chunk_spread(pos)?)
    }
}

impl Index<ivec3> for Chunk {
    type Output = Tile;

    fn index(&self, index: ivec3) -> &Self::Output {
        &self.tiles[chunk_spread(index).expect("chunk index out of bounds")]
    }
}

impl IndexMut<ivec3> for Chunk {
    fn index_mut(&mut self, index: ivec3) -> &mut Self::Output {
        &mut self.tiles[chunk_spread(index).expect("chunk index out of bounds")]
    }
}

impl Chunk {
    pub fn keys() -> impl Iterator<Item = ivec3> { chunk_indices() }

    // TODO: Use foreach instead, this iterator is turbo-ugly
    pub fn iter(&self) -> impl Iterator<Item = (ivec3, &Tile)> {
        // Is there something like Kotlin's `associateWith`?
        chunk_indices().map(|p| (p, &self[p]))
    }
}
