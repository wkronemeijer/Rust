use std::ops::Index;
use std::ops::IndexMut;

use super::tile::Tile;
use crate::core::iter::IntegerTripleIter;

//////////////
// Indexing //
//////////////

/// The length in one dimension of a 3D chunk.
pub const CHUNK_DIM: usize = 16;

const CHUNK_DIM_U8: u8 = CHUNK_DIM as u8;

/// The volume of a 3D chunk.
pub const CHUNK_VOLUME: usize = CHUNK_DIM * CHUNK_DIM * CHUNK_DIM;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct ChunkToTileIndex {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl ChunkToTileIndex {
    pub fn new(x: u8, y: u8, z: u8) -> Option<Self> {
        if z < CHUNK_DIM_U8 && y < CHUNK_DIM_U8 && x < CHUNK_DIM_U8 {
            Some(ChunkToTileIndex { x, y, z })
        } else {
            None
        }
    }

    #[inline]
    pub fn for_each<F: FnMut(Self)>(mut body: F) {
        for z in 0..CHUNK_DIM_U8 {
            for y in 0..CHUNK_DIM_U8 {
                for x in 0..CHUNK_DIM_U8 {
                    body(ChunkToTileIndex { x, y, z })
                }
            }
        }
    }

    pub fn every() -> impl Iterator<Item = ChunkToTileIndex> {
        IntegerTripleIter::new(CHUNK_DIM_U8, CHUNK_DIM_U8, CHUNK_DIM_U8)
            .map(|(x, y, z)| ChunkToTileIndex { x, y, z })
    }

    pub fn spread(self) -> usize {
        let x = usize::from(self.x);
        let y = CHUNK_DIM * usize::from(self.y);
        let z = CHUNK_DIM * CHUNK_DIM * usize::from(self.z);
        // ↓ this can only overflow on a 16-bit usize platform
        x + y + z
    }
}

pub type ChunkCoords = ChunkToTileIndex;

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

    pub fn get(&self, pos: ChunkToTileIndex) -> Option<&Tile> {
        self.tiles.get(pos.spread())
    }

    pub fn get_mut(&mut self, pos: ChunkToTileIndex) -> Option<&mut Tile> {
        self.tiles.get_mut(pos.spread())
    }
}

impl Index<ChunkToTileIndex> for Chunk {
    type Output = Tile;

    fn index(&self, index: ChunkToTileIndex) -> &Self::Output {
        &self.tiles[index.spread()]
    }
}

impl IndexMut<ChunkToTileIndex> for Chunk {
    fn index_mut(&mut self, index: ChunkToTileIndex) -> &mut Self::Output {
        &mut self.tiles[index.spread()]
    }
}

impl Chunk {
    /// Iterates over all tiles, also passing the location.
    ///
    /// API modeled after [`enumerate`](std::iter::Iterator::enumerate).
    pub fn for_each_tile<F: FnMut(ChunkToTileIndex, &Tile)>(
        &self,
        mut func: F,
    ) {
        ChunkToTileIndex::for_each(|pos| func(pos, &self[pos]))
    }

    /// Iterates mutably over all tiles, also passing the location.
    ///
    /// API modeled after [`enumerate`](std::iter::Iterator::enumerate).
    pub fn for_each_tile_mut<F: FnMut(ChunkToTileIndex, &mut Tile)>(
        &mut self,
        mut func: F,
    ) {
        ChunkToTileIndex::for_each(|pos| func(pos, &mut self[pos]))
    }
}
