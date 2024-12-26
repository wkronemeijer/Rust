use std::time::Duration;

////////////
// Update //
////////////

pub struct DeltaTime {
    pub seconds: f64,
}

impl DeltaTime {
    pub fn from_duration(dt: Duration) -> Self {
        DeltaTime { seconds: dt.as_secs_f64() }
    }
}

pub trait Update {
    fn update(&mut self, dt: DeltaTime);
}

//////////
// Tick //
//////////

pub trait Tick {
    fn tick(&mut self);
}
