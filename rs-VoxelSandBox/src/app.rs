//! Ties the simulation, rendering, audio (soon&trade;) and input together.

use std::default::Default;
use std::f32::consts::PI;
use std::f32::consts::TAU;
use std::time::Duration;
use std::time::Instant;

use glium::Display;
use glium::Surface;
use glium::glutin::surface::WindowSurface;
use winit::application::ApplicationHandler;
use winit::event::DeviceEvent;
use winit::event::DeviceId;
use winit::event::ElementState::Pressed;
use winit::event::KeyEvent;
use winit::event::MouseButton;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
use winit::window::CursorGrabMode;
use winit::window::Fullscreen;
use winit::window::Window;
use winit::window::WindowId;

use crate::assets::load_icon_png;
use crate::camera::Camera;
use crate::core::AspectRatioExt as _;
use crate::display::state::Renderer;
use crate::domain::SECONDS_PER_TICK;
use crate::domain::TICK_DURATION;
use crate::domain::game::Game;
use crate::input::InputState;
use crate::input::VirtualButton;
use crate::mat4;
use crate::vec2;
use crate::vec3;

#[derive(Debug, Default, Clone, Copy)]
pub enum CursorState {
    #[default]
    Free,
    Grabbed,
}

pub struct Application {
    window: Window,
    cursor_state: CursorState,
    input: InputState,

    display: Display<WindowSurface>,
    renderer: Renderer,
    camera: Camera,

    game: Game,

    last_draw: Instant,
    last_update: Instant,
    last_tick: Instant,
}

impl Application {
    pub fn new(
        window: Window,
        display: Display<WindowSurface>,
    ) -> crate::Result<Self> {
        let game = Game::new();
        let renderer = Renderer::new(&display)?;
        window.set_window_icon(Some(load_icon_png()?));
        Ok(Application {
            window,
            display,
            renderer,
            game,
            cursor_state: CursorState::default(),
            camera: Camera::new(),
            input: InputState::new(),
            last_update: Instant::now(),
            last_tick: Instant::now(),
            last_draw: Instant::now(),
        })
    }
}
const FOV_Y_RADIANS: f32 = 90.0 * PI / 180.0;

// Drawing logic
impl Application {
    fn projection(&self) -> mat4 {
        let aspect_ratio = self.window.inner_size().aspect_ratio();
        let z_near = 0.001;
        let z_far = 1000.0;

        mat4::perspective_rh_gl(FOV_Y_RADIANS, aspect_ratio, z_near, z_far)
    }

    pub fn draw(&mut self) -> crate::Result {
        self.renderer.update(&self.display, &self.game)?;

        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        let view = self.camera.view();
        let projection = self.projection();
        self.renderer.draw(&mut frame, view, projection)?;

        frame.finish()?;
        self.last_draw = Instant::now();
        Ok(())
    }
}

////////////////
// Tick logic //
////////////////

const PLAYER_UNITS_PER_SECOND: f32 = 5.0;
const ANGLE_PER_SECOND: f32 = PI / 2.0;
const ASPECT_CORRECTION: vec2 = vec2(1.0, 1.0 / FOV_Y_RADIANS);
const ANGLE_PER_MOUSE_UNIT: f32 = TAU / 10_000.0;

impl Application {
    pub fn projected_next_tick(&self) -> Instant {
        self.last_tick + TICK_DURATION
    }

    pub fn tick(&mut self) {
        self.game.tick();

        // TODO: Do we do self.camera.tick()?
        // Or tie it to an entity
        // Or both and add optional detach for debugging
        // Updating view angle on tick will feel really bad

        self.camera.change_camera_position(
            self.input.wishdir() * PLAYER_UNITS_PER_SECOND * SECONDS_PER_TICK,
        );

        self.camera.change_lookangles(
            self.input.wishlook() *
                ANGLE_PER_SECOND *
                SECONDS_PER_TICK *
                ASPECT_CORRECTION,
        );

        /////////////
        // Cleanup //
        /////////////

        self.input.clear();
    }

    /// Tries to tick at most once.
    pub fn try_tick(&mut self) {
        let now = Instant::now();
        if (now - self.last_tick) >= TICK_DURATION {
            self.tick();
            self.last_tick = now;
        }
    }
}

//////////////////
// Update logic //
//////////////////

impl Application {
    fn update(&mut self, _: Duration) {
        if matches!(self.cursor_state, CursorState::Grabbed) {
            self.camera.change_lookangles(
                self.input.mouse_delta().as_vec2() *
                    ANGLE_PER_MOUSE_UNIT *
                    ASPECT_CORRECTION,
            );
        }
    }

    fn do_update(&mut self) {
        let now = Instant::now();
        let dt = now - self.last_update;
        self.update(dt);
        self.last_update = now;
    }
}

/////////////////////
// Mousegrab logic //
/////////////////////

impl Application {
    fn grab_cursor(&mut self) {
        if self.window.set_cursor_grab(CursorGrabMode::Confined).is_ok() {
            self.window.set_cursor_visible(false);
            self.cursor_state = CursorState::Grabbed;
        }
    }

    fn free_cursor(&mut self) {
        let _ = self.window.set_cursor_grab(CursorGrabMode::None);
        self.window.set_cursor_visible(true);
        self.cursor_state = CursorState::Free;
    }
}

////////////////////////////
// Handling window events //
////////////////////////////

impl ApplicationHandler for Application {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        use WindowEvent::*;
        match event {
            /////////////
            // Drawing //
            /////////////
            RedrawRequested => {
                self.try_tick();
                self.do_update();
                self.draw().expect("drawing failed");
            }
            Resized(inner_size) => self.display.resize(inner_size.into()),
            ///////////
            // Input //
            ///////////
            Focused(true) => self.grab_cursor(),
            KeyboardInput {
                event:
                    key_event @ KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        repeat: false,
                        state,
                        ..
                    },
                ..
            } => match (key, state) {
                #[cfg(debug_assertions)]
                (KeyCode::F8, Pressed) => event_loop.exit(),
                (KeyCode::F10, Pressed) => self.free_cursor(),
                (KeyCode::F11, Pressed) => {
                    self.window.set_fullscreen(match self.window.fullscreen() {
                        None => Some(Fullscreen::Borderless(None)),
                        Some(_) => None,
                    })
                }
                _ => self.input.process_key_event(key_event),
            },
            MouseInput { button, state, .. } => match (button, state) {
                (MouseButton::Left, Pressed) => {
                    self.grab_cursor();
                }
                _ => {}
            },
            CloseRequested => event_loop.exit(),
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _: &ActiveEventLoop,
        _: DeviceId,
        event: DeviceEvent,
    ) {
        use DeviceEvent::*;
        match event {
            MouseMotion { delta } => {
                // MouseMotion events are not affect by Windows Mouse Settings
                // according to https://github.com/bevyengine/bevy/issues/1149
                // ...and they continue when you hit the edge of the screen.
                self.input.process_motion_event(delta);
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            self.projected_next_tick(),
        ));
        self.window.request_redraw();
    }

    fn resumed(&mut self, _: &ActiveEventLoop) {
        println!("<resumed>");
    }
}
