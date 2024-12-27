use std::ops::Index;
use std::ops::IndexMut;

use super::handles::CHUNK_VOLUME;
use super::handles::ChunkToTileIndex;
use super::tile::Tile;
use crate::core::memory_usage::AllocatedSize;

//////////////
// Indexing //
//////////////

pub type ChunkIndex = ChunkToTileIndex;
pub type TileIndex = ChunkToTileIndex;

///////////
// Chunk //
///////////

/// A cube containing [CHUNK_VOLUME] tiles.
#[derive(Debug, Clone)]
pub struct Chunk {
    /// Invariant: contains exactly CHUNK_VOLUME elements
    tiles: Box<[Tile; CHUNK_VOLUME]>,
    // is_dirty flag?
    // Hash the chunk for changes?
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

    pub fn get(&self, pos: ChunkIndex) -> Option<&Tile> {
        self.tiles.get(pos.spread())
    }

    pub fn get_mut(&mut self, pos: ChunkIndex) -> Option<&mut Tile> {
        self.tiles.get_mut(pos.spread())
    }

    pub fn retained_size(&self) -> usize {
        self.tiles.len() * size_of::<Tile>()
    }
}

impl Index<ChunkIndex> for Chunk {
    type Output = Tile;

    fn index(&self, index: ChunkIndex) -> &Self::Output {
        &self.tiles[index.spread()]
    }
}

impl IndexMut<ChunkIndex> for Chunk {
    fn index_mut(&mut self, index: ChunkIndex) -> &mut Self::Output {
        &mut self.tiles[index.spread()]
    }
}

impl Chunk {
    /// Iterates over all tiles, also passing the location.
    ///
    /// API modeled after [`enumerate`](std::iter::Iterator::enumerate).
    pub fn for_each_tile<F: FnMut(ChunkIndex, &Tile)>(&self, mut func: F) {
        ChunkToTileIndex::for_each(|pos| func(pos, &self[pos]))
    }

    /// Iterates mutably over all tiles, also passing the location.
    ///
    /// API modeled after [`enumerate`](std::iter::Iterator::enumerate).
    pub fn for_each_tile_mut<F: FnMut(ChunkIndex, &mut Tile)>(
        &mut self,
        mut func: F,
    ) {
        ChunkToTileIndex::for_each(|pos| func(pos, &mut self[pos]))
    }
}

impl AllocatedSize for Chunk {
    fn allocated_size(&self) -> usize { self.tiles.allocated_size() }
}
