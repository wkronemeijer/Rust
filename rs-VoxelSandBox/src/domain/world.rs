use std::collections::HashMap;

use num_traits::Euclid;
use rand::Rng;
use rand::thread_rng;

use super::chunk::CHUNK_DIM;
use super::chunk::Chunk;
use super::chunk::ChunkToTileIndex;
use super::tile::Tile;
use super::traits::DeltaTime;
use crate::core::iter::IntegerTripleIter;
use crate::vec3;

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

    /// Returns the world-space origin of the associated chunk.
    pub fn world_origin(self) -> vec3 {
        let x = self.x as f32;
        let y = self.y as f32;
        let z = self.z as f32;
        vec3(x, y, z) * CHUNK_DIM as f32
    }
}

////////////////
// World Size //
////////////////

#[derive(Debug, Clone, Copy)]
pub struct WorldChunkSize {
    // All exclusive!
    pub max_x: u16,
    pub max_y: u16,
    pub max_z: u16,
}

impl WorldChunkSize {
    pub fn new(x_size: u16, y_size: u16, z_size: u16) -> Self {
        WorldChunkSize { max_x: x_size, max_y: y_size, max_z: z_size }
    }

    pub fn contains(self, p: WorldToChunkIndex) -> bool {
        p.x < self.max_x && p.y < self.max_y && p.z < self.max_z
    }

    /// Iterates through all valid points inside this size.
    pub fn span(self) -> impl Iterator<Item = WorldToChunkIndex> {
        IntegerTripleIter::new(self.max_x, self.max_y, self.max_z)
            .map(|(x, y, z)| WorldToChunkIndex { x, y, z })
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

//////////////////////
// WorldToTileIndex //
//////////////////////

// So many similar structs
// I wonder if I can abstract over them all...
// and whether that is a good idea ofc

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldToTileIndex {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

const CHUNK_DIM_U32: u32 = CHUNK_DIM as u32;

impl WorldToTileIndex {
    pub fn new(x: u32, y: u32, z: u32) -> Self { WorldToTileIndex { x, y, z } }

    pub fn split(self) -> (WorldToChunkIndex, ChunkToTileIndex) {
        let (cx, tx) = self.x.div_rem_euclid(&CHUNK_DIM_U32);
        let (cy, ty) = self.y.div_rem_euclid(&CHUNK_DIM_U32);
        let (cz, tz) = self.z.div_rem_euclid(&CHUNK_DIM_U32);
        (
            WorldToChunkIndex::new(cx as u16, cy as u16, cz as u16),
            ChunkToTileIndex::new(tx as u8, ty as u8, tz as u8).unwrap(),
        )
    }
}

///////////////////
// WorldTileSize //
///////////////////

#[derive(Debug, Clone, Copy)]
pub struct WorldTileSize {
    // All exclusive!
    pub max_x: u32,
    pub max_y: u32,
    pub max_z: u32,
}

impl WorldTileSize {
    pub fn new(x_size: u32, y_size: u32, z_size: u32) -> Self {
        WorldTileSize { max_x: x_size, max_y: y_size, max_z: z_size }
    }

    pub fn from_chunk_size(
        WorldChunkSize { max_x, max_y, max_z }: WorldChunkSize,
    ) -> Self {
        WorldTileSize {
            max_x: max_x as u32 * CHUNK_DIM_U32,
            max_y: max_y as u32 * CHUNK_DIM_U32,
            max_z: max_z as u32 * CHUNK_DIM_U32,
        }
    }

    pub fn contains(self, p: WorldToTileIndex) -> bool {
        p.x < self.max_x && p.y < self.max_y && p.z < self.max_z
    }

    /// Iterates through all valid points inside this size.
    pub fn span(self) -> impl Iterator<Item = WorldToTileIndex> {
        IntegerTripleIter::new(self.max_x, self.max_y, self.max_z)
            .map(|(x, y, z)| WorldToTileIndex { x, y, z })
    }
}

///////////
// World //
///////////

#[derive(Debug)]
pub struct World {
    // One chunk for now
    chunk_size: WorldChunkSize,
    tile_size: WorldTileSize,
    chunks: HashMap<WorldToChunkIndex, Chunk>,
}

fn fill_world(world: &mut World) {
    // Populate chunks
    // Maybe move this to a seperate function? and called seperately?
    let mut rng = thread_rng();

    for tile_idx in world.tile_size.span() {
        let Some(tile) = world.get_tile_mut(tile_idx) else { continue };

        const STONE_END: u32 = 10;
        const DIRT_END: u32 = 25;
        const GRASS_END: u32 = 40;

        let tile_z = tile_idx.z;

        let gradient = 1.0 -
            ((tile_z as f64 - STONE_END as f64) /
                (GRASS_END as f64 - STONE_END as f64))
                .clamp(0.0, 1.0);

        if !(tile_z > GRASS_END) &&
            (tile_z < STONE_END || rng.gen_bool(gradient))
        {
            if tile_z < STONE_END {
                *tile = Tile::Stone
            } else if tile_z < DIRT_END {
                *tile = Tile::Dirt
            } else if tile_z < GRASS_END {
                *tile = Tile::Grass
            }
        }
    }
}

impl World {
    pub fn new() -> Self {
        let chunk_size = WorldChunkSize::new(4, 4, 4);
        let mut world = World {
            chunk_size,
            tile_size: WorldTileSize::from_chunk_size(chunk_size),
            chunks: HashMap::new(),
        };
        for chunk_idx in world.chunk_size.span() {
            world.chunks.insert(chunk_idx, Chunk::new());
        }
        fill_world(&mut world);
        world
    }

    pub fn chunk_size(&self) -> WorldChunkSize { self.chunk_size }

    pub fn tile_size(&self) -> WorldTileSize { self.tile_size }

    pub fn get_chunk(&self, idx: WorldToChunkIndex) -> Option<&Chunk> {
        self.chunks.get(&idx)
    }

    pub fn get_chunk_mut(
        &mut self,
        idx: WorldToChunkIndex,
    ) -> Option<&mut Chunk> {
        if self.chunk_size.contains(idx) {
            self.chunks.get_mut(&idx)
        } else {
            None
        }
    }

    pub fn get_tile(&self, idx: WorldToTileIndex) -> Option<&Tile> {
        let (chunk_idx, tile_idx) = idx.split();
        self.get_chunk(chunk_idx)?.get(tile_idx)
    }

    pub fn get_tile_mut(&mut self, idx: WorldToTileIndex) -> Option<&mut Tile> {
        let (chunk_idx, tile_idx) = idx.split();
        self.get_chunk_mut(chunk_idx)?.get_mut(tile_idx)
    }

    pub fn update(&mut self, _: DeltaTime) {}

    pub fn tick(&mut self) {}
}
