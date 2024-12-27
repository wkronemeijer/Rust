use super::traits::DeltaTime;
use super::world::World;

pub struct Game {
    pub world: World,
}

impl Game {
    pub fn new() -> Self { Game { world: World::new() } }

    pub fn update(&mut self, dt: DeltaTime) {
        self.world.update(dt);
        // no-op
    }

    pub fn tick(&mut self) { self.world.tick(); }
}
