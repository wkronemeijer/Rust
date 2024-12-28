use std::marker::PhantomData;

///////////////////
// World -> Tile //
///////////////////

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldToTileIndex {
    private: PhantomData<()>,
}

// TODO: Universe -> Dimension -> World -> Chunk -> Tile
