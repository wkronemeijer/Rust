// TODO: Split between "continous" input checked every frame (like moving forward)
// And "event" input anywhere during the frame (like jumping)

use winit::event::ElementState;
use winit::event::ElementState::Pressed;
use winit::keyboard::KeyCode;

#[derive(Debug, Default, Clone)]
pub struct InputState {
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,

    pub rotate_up: bool,
    pub rotate_down: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
}

impl InputState {
    pub fn new() -> Self { Default::default() }

    /// Updates state based on pressed key.
    /// Returns [`Some`] if state was updated, [`None`] if the key was not recognized.
    pub fn process(&mut self, key: KeyCode, state: ElementState) -> Option<()> {
        use KeyCode::*;
        let is_pressed = matches!(state, Pressed);
        Some(match key {
            KeyW => self.move_forward = is_pressed,
            KeyS => self.move_backward = is_pressed,
            KeyA => self.move_left = is_pressed,
            KeyD => self.move_right = is_pressed,
            KeyE => self.move_up = is_pressed,
            KeyQ => self.move_down = is_pressed,

            ArrowUp => self.rotate_up = is_pressed,
            ArrowDown => self.rotate_down = is_pressed,
            ArrowLeft => self.rotate_left = is_pressed,
            ArrowRight => self.rotate_right = is_pressed,

            _ => return None,
        })
    }
}
