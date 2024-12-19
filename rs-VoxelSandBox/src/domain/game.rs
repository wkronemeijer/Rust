use std::time::Duration;

use super::world::World;

pub struct Game {
    pub world: World,
}

impl Game {
    pub fn new() -> Self { Game { world: World::new() } }

    pub fn update(&mut self, _: Duration) {
        // no-op
    }

    pub fn tick(&mut self) { self.world.tick(); }
}
