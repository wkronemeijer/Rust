pub mod app;
pub mod assets {
    pub mod fonts;
}
pub mod core {
    pub mod bitflags;
    pub mod genmap;
    pub mod slice;
}
pub mod events;
pub mod game;
pub mod ideas;
pub mod items;
pub mod rng;
pub mod stats;

pub type Error = ::anyhow::Error;

pub type Result<T = (), E = Error> = ::std::result::Result<T, E>;

pub const APP_NAME: &str = "SRPG Arena II";
