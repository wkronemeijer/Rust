//! Ties the simulation, rendering, audio (soon&trade;) and input together.

use std::default::Default;
use std::f32::consts::PI;
use std::time::Instant;

use glium::glutin::surface::WindowSurface;
use glium::texture::CompressedTexture2d;
use glium::Depth;
use glium::DepthTest;
use glium::Display;
use glium::DrawParameters;
use glium::Program;
use glium::Surface;
use winit::application::ApplicationHandler;
use winit::event::DeviceEvent;
use winit::event::DeviceId;
use winit::event::ElementState::Pressed;
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
use crate::assets::load_terrain_png;
use crate::camera::Camera;
use crate::core::AspectRatioExt as _;
use crate::display::shader::chunk_mesh;
use crate::display::shader::chunk_program;
use crate::display::shader::chunk_uniforms;
use crate::display::shader::ChunkMesh;
use crate::display::Mesh;
use crate::domain::world::World;
use crate::domain::SECONDS_PER_TICK;
use crate::domain::TICK_DURATION;
use crate::input::InputState;
use crate::input::VirtualButton;
use crate::mat4;
use crate::vec2;
use crate::vec3;

pub struct Application {
    window: Window,
    display: Display<WindowSurface>,

    program: Program,
    options: DrawParameters<'static>,
    mesh: ChunkMesh,
    texture: CompressedTexture2d,

    world: World,
    camera: Camera,
    input: InputState,
    last_tick: Instant,
}

impl Application {
    pub fn new(
        window: Window,
        display: Display<WindowSurface>,
    ) -> crate::Result<Self> {
        let world = World::new();
        let gl = &display;

        // TODO: Move to RenderState
        let program = chunk_program(gl)?;
        let options = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let mesh = chunk_mesh(gl, &world.chunk)?;
        let texture = load_terrain_png(gl)?;

        let icon = load_icon_png()?;
        window.set_window_icon(Some(icon));

        Ok(Application {
            window,
            display,
            program,
            options,
            mesh,
            texture,
            world,
            camera: Camera::new(),
            input: InputState::new(),
            last_tick: Instant::now(),
        })
    }
}

// Drawing logic
impl Application {
    const FOV_Y_RADIANS: f32 = 90.0 * PI / 180.0;

    fn projection(&self) -> mat4 {
        let aspect_ratio = self.window.inner_size().aspect_ratio();
        let z_near = 0.001;
        let z_far = 1000.0;

        mat4::perspective_rh_gl(
            Self::FOV_Y_RADIANS,
            aspect_ratio,
            z_near,
            z_far,
        )
    }

    pub fn draw(&self) -> crate::Result {
        // I need to put it somewhere else

        let mut frame = self.display.draw();

        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.clear_depth(1.0);

        let Mesh { vertices, indices } = &self.mesh;
        let model = mat4::IDENTITY; // just in place (for now)
        let view = self.camera.view();
        let projection = self.projection();
        let mvp = projection * view * model;

        let uniforms = chunk_uniforms(&self.texture, &mvp);
        frame.draw(
            vertices,
            indices,
            &self.program,
            &uniforms,
            &self.options,
        )?;

        Ok(frame.finish()?)
    }
}

////////////////
// Tick logic //
////////////////

/// Extracts (Δx, Δy, Δz) from input
fn wisdir(input: &InputState) -> vec3 {
    use VirtualButton::*;
    let mut wishdir = vec3::ZERO;
    if input.is_pressed(MoveForward) {
        wishdir += vec3::Y;
    }
    if input.is_pressed(MoveBackward) {
        wishdir -= vec3::Y;
    }
    if input.is_pressed(MoveRight) {
        wishdir += vec3::X;
    }
    if input.is_pressed(MoveLeft) {
        wishdir -= vec3::X;
    }
    if input.is_pressed(MoveUp) {
        wishdir += vec3::Z;
    }
    if input.is_pressed(MoveDown) {
        wishdir -= vec3::Z;
    }
    wishdir.normalize_or_zero()
}

/// Extracts (Δyaw, Δpitch) from input
fn wishlook(input: &InputState) -> vec2 {
    use VirtualButton::*;
    let mut wishlook = vec2::ZERO;
    if input.is_pressed(RotateLeft) {
        wishlook += vec2::X;
    }
    if input.is_pressed(RotateRight) {
        wishlook -= vec2::X;
    }
    if input.is_pressed(RotateUp) {
        wishlook += vec2::Y;
    }
    if input.is_pressed(RotateDown) {
        wishlook -= vec2::Y;
    }
    wishlook.normalize_or_zero()
}

const PLAYER_UNITS_PER_SECOND: f32 = 5.0;
const ANGLE_PER_SECOND: f32 = PI / 2.0;

impl Application {
    pub fn projected_next_tick(&self) -> Instant {
        self.last_tick + TICK_DURATION
    }

    /// Tries to tick at most once.
    pub fn try_tick(&mut self) {
        let now = Instant::now();
        if (now - self.last_tick) >= TICK_DURATION {
            self.tick();
            self.last_tick = now;
        }
    }

    pub fn tick(&mut self) {
        self.world.tick();
        // TODO: Do we do self.camera.tick()?
        // Or tie it to an entity
        // Or both and add optional detach for debugging
        // Updating view angle on tick will feel really bad

        //////////////
        // Movement //
        //////////////

        self.camera.change_camera_position(
            wisdir(&self.input) * PLAYER_UNITS_PER_SECOND * SECONDS_PER_TICK,
        );

        //////////////
        // Rotation //
        //////////////
        // Keyboard rotation

        // (Δyaw, Δpitch)
        let mut wishlook =
            wishlook(&self.input) * ANGLE_PER_SECOND * SECONDS_PER_TICK;

        // Without this vertical sens feels too fast
        wishlook.y /= Self::FOV_Y_RADIANS;

        self.camera.change_yaw(wishlook.x);
        self.camera.change_pitch(wishlook.y);

        /////////////
        // Cleanup //
        /////////////

        self.input.clear();
    }
}

//////////////////
// Update logic //
//////////////////

impl Application {}

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
                self.draw().expect("drawing failed");
            }
            Resized(inner_size) => self.display.resize(inner_size.into()),
            ///////////
            // Input //
            ///////////
            KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(code) => match (code, event.state) {
                    ///////////
                    // Debug //
                    ///////////
                    (KeyCode::Escape, Pressed) => {
                        if let Err(e) =
                            self.window.set_cursor_grab(CursorGrabMode::None)
                        {
                            eprintln!("failed to set cursor: {e}");
                        }
                    }
                    #[cfg(debug_assertions)]
                    (KeyCode::F8, Pressed) => event_loop.exit(),
                    (KeyCode::F10, Pressed) => {
                        println!("TODO: toggle menu")
                    }
                    (KeyCode::F11, Pressed) => self.window.set_fullscreen(
                        match self.window.fullscreen() {
                            None => Some(Fullscreen::Borderless(None)),
                            Some(_) => None,
                        },
                    ),
                    _ => self.input.process_key_event(event),
                },
                PhysicalKey::Unidentified(c) => {
                    println!("warning: unidentified key {c:?}");
                }
            },
            CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _: &ActiveEventLoop,
        device: DeviceId,
        event: DeviceEvent,
    ) {
        use DeviceEvent::*;
        match event {
            MouseMotion { delta: (_, _) } => {
                // MouseMotion events are not affect by Windows Mouse Settings
                // according to https://github.com/bevyengine/bevy/issues/1149
                // ...and they continue when you hit the edge of the screen.
            }
            Added => {
                println!("device #{device:?} added");
            }
            Removed => {
                println!("device #{device:?} removed");
            }
            // Motion { axis, value } => {
            //     println!("axis #{axis} set to {value:?} on device #{device:?}");
            // }
            // Button { button, state } => {
            //     println!(
            //         "button #{button} set to {state:?} on device #{device:?}"
            //     );
            // }
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
