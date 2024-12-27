use std::marker::PhantomData;

///////////////////
// Chunk -> Tile //
///////////////////

/// The length in one dimension of a 3D chunk.
pub const CHUNK_DIM: usize = 16;
/// The volume of a 3D chunk.
pub const CHUNK_VOLUME: usize = CHUNK_DIM * CHUNK_DIM * CHUNK_DIM;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChunkToTileIndex {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    private: PhantomData<()>,
}

const CHUNK_DIM_U8: u8 = CHUNK_DIM as u8;

impl ChunkToTileIndex {
    pub fn new(x: u8, y: u8, z: u8) -> Option<Self> {
        if z < CHUNK_DIM_U8 && y < CHUNK_DIM_U8 && x < CHUNK_DIM_U8 {
            Some(ChunkToTileIndex { x, y, z, private: PhantomData })
        } else {
            None
        }
    }

    #[inline]
    pub fn for_each<F: FnMut(Self)>(mut body: F) {
        for z in 0..CHUNK_DIM_U8 {
            for y in 0..CHUNK_DIM_U8 {
                for x in 0..CHUNK_DIM_U8 {
                    body(ChunkToTileIndex { x, y, z, private: PhantomData })
                }
            }
        }
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

////////////////////
// World -> Chunk //
////////////////////

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldToChunkIndex {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    private: PhantomData<()>,
}

///////////////////
// World -> Tile //
///////////////////

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldToTileIndex {
    private: PhantomData<()>,
}

// TODO: Universe -> Dimension -> World -> Chunk -> Tile
