use std::time::Duration;

////////////
// Update //
////////////

/// Fractional number of seconds since the last update.
/// Always greater than or equal to [`DeltaTime::MIN`].
#[derive(Debug, Clone, Copy)]
pub struct DeltaTime {
    seconds: f64,
}

impl DeltaTime {
    /// Minimum time that must elapse before an update happens.
    /// This is to prevent accuracy issues from very small Δt values.
    pub const MIN: f64 = 0.000_001; // 1µs

    /// Constructs a [`DeltaTime`] value from a [`Duration`].
    /// Returns [`None`] if the duration is too short.
    pub fn from_duration(dt: Duration) -> Option<Self> {
        let seconds = dt.as_secs_f64();
        if seconds >= Self::MIN { Some(DeltaTime { seconds }) } else { None }
    }

    /// Returns the fractional number of seconds since the last frame as a float.
    pub fn as_secs_f32(self) -> f32 { self.seconds as f32 }

    /// Returns the fractional number of seconds since the last frame as a double.
    pub fn as_secs_f64(&self) -> f64 { self.seconds }
}

pub trait Update {
    fn update(&mut self, dt: DeltaTime);
}

//////////
// Tick //
//////////

pub trait Tick {
    /// Save the current fields to previous fields.
    /// This allows the render function to interpolate the movement between ticks.
    // fn save_prev(&mut self);
    // TODO: Implement save_prev();
    // We wait so we can see the choppy movement first

    /// Update the simulation by 1 fixed timestep.
    fn tick(&mut self);
}
