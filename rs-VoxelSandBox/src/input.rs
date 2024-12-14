//! Provides items for managing user input.

// TODO: Split between "continous" input checked every frame (like moving forward)
// And "event" input anywhere during the frame (like jumping)

use std::collections::HashSet;
use std::mem::take;

use winit::event::ElementState;
use winit::event::KeyEvent;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;

use crate::dvec2;

////////////////////
// Physical Input //
////////////////////

#[derive(Debug, Default, Clone)]
pub struct InputState {
    // I wonder if there is a faster set we can use
    pressed_right_now: HashSet<KeyCode>,
    pressed_since_last_time: HashSet<KeyCode>,

    mouse_delta: dvec2,
}

impl InputState {
    pub fn new() -> Self { Default::default() }

    ////////////////
    // Processing //
    ////////////////

    fn process_physical_key(&mut self, key: KeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => self.pressed_right_now.insert(key),
            ElementState::Released => self.pressed_right_now.remove(&key),
        };
    }

    pub fn process_key_event(&mut self, event: KeyEvent) {
        // For now, we just use physical key
        match event {
            KeyEvent {
                physical_key: PhysicalKey::Code(code),
                repeat: false,
                state,
                ..
            } => self.process_physical_key(code, state),
            _ => {}
        };
    }

    pub fn process_motion_event(&mut self, (dx, dy): (f64, f64)) {
        // Window's +X is -Yaw
        // Window's +Y is -Pitch
        self.mouse_delta.x -= dx;
        self.mouse_delta.y -= dy;
    }

    ////////////
    // Access //
    ////////////

    pub fn key_is_pressed(&self, key: KeyCode) -> bool {
        self.pressed_right_now.contains(&key)
    }

    pub fn mouse_delta(&mut self) -> dvec2 { take(&mut self.mouse_delta) }

    /////////////
    // Cleanup //
    /////////////

    /// Clears the input since the last frame.
    pub fn clear(&mut self) {
        // Should this be the drop method of some associated struct?
        // Then the method could only be called once...
        // HMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
        self.pressed_since_last_time.clear();
    }
}

///////////////////
// Virtual Input //
///////////////////

#[derive(Debug, Clone, Copy)]
pub enum VirtualButton {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,

    RotateUp,
    RotateDown,
    RotateLeft,
    RotateRight,
}

impl VirtualButton {
    fn check(self, state: &InputState) -> bool {
        use KeyCode::*;
        use VirtualButton::*;
        match self {
            MoveForward => state.key_is_pressed(KeyW),
            MoveBackward => state.key_is_pressed(KeyS),
            MoveLeft => state.key_is_pressed(KeyA),
            MoveRight => state.key_is_pressed(KeyD),
            MoveUp => state.key_is_pressed(KeyE) || state.key_is_pressed(Space),
            MoveDown => state.key_is_pressed(KeyQ),
            RotateUp => state.key_is_pressed(ArrowUp),
            RotateDown => state.key_is_pressed(ArrowDown),
            RotateLeft => state.key_is_pressed(ArrowLeft),
            RotateRight => state.key_is_pressed(ArrowRight),
        }
    }
}

impl InputState {
    pub fn is_pressed(&self, button: VirtualButton) -> bool {
        button.check(self)
    }
}
