//! Contains all code primarily concerned with drawing to a display.

pub mod shader;

pub struct Mesh<V, I> {
    pub vertices: V,
    pub indices: I,
}
