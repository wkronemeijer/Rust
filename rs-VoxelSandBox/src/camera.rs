// https://en.wikipedia.org/wiki/Aircraft_flight_dynamics#Transformations_(Euler_angles)
// mentions ψ = yaw, θ = pitch (and φ = roll)

use std::f32::consts::FRAC_PI_2;
use std::f32::consts::TAU;

use glam::EulerRot;

use crate::mat3;
use crate::mat4;
use crate::vec2;
use crate::vec3;

/// By default looks down +Y
#[derive(Debug)]
pub struct Camera {
    position: vec3,
    /// + is left, - is right
    yaw: f32,
    /// + is up, - is down
    pitch: f32,
    // no roll!
}

impl Camera {
    pub fn new() -> Self {
        Camera { position: vec3::ZERO, pitch: 0.0, yaw: 0.0 }
    }

    pub fn view(&self) -> mat4 {
        let Camera { yaw, pitch, .. } = *self;
        let dir = mat3::from_euler(EulerRot::ZXY, yaw, pitch, 0.0) * vec3::Y;
        mat4::look_to_rh(self.position, dir, vec3::Z)
    }

    /// Rotates movement so it is in accordance with an unrotated camera. So with:
    /// * yaw == 0, it goes through unchanged  
    /// * yaw == &tau;/4 it is rotated to the left  
    /// * yaw == &tau;/2 it goes in reverse  
    pub fn change_camera_position(&mut self, wish_displacement: vec3) {
        self.position += mat3::from_rotation_z(self.yaw) * wish_displacement;
    }

    pub fn change_yaw(&mut self, delta: f32) {
        self.yaw = (self.yaw + delta).rem_euclid(TAU);
    }

    // -ε to prevent flipping the camera
    const MAX_PITCH: f32 = FRAC_PI_2 - f32::EPSILON;
    const MIN_PITCH: f32 = -Self::MAX_PITCH;

    pub fn change_pitch(&mut self, delta: f32) {
        self.pitch =
            (self.pitch + delta).clamp(Self::MIN_PITCH, Self::MAX_PITCH);
    }

    pub fn change_lookangles(&mut self, delta: vec2) {
        self.change_yaw(delta.x);
        self.change_pitch(delta.y);
    }
}
