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

// Tick logic
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

    const PLAYER_UNITS_PER_SECOND: f32 = 5.0;
    const ANGLE_PER_SECOND: f32 = PI / 2.0;

    pub fn tick(&mut self) {
        self.world.tick();
        // TODO: Do we do self.camera.tick()?
        // Or tie it to an entity
        // Or both and add optional detach for debugging

        //////////////
        // Movement //
        //////////////
        // Move to input

        let mut wishdir = vec3::ZERO;
        if self.input.move_forward {
            wishdir += vec3::Y;
        }
        if self.input.move_backward {
            wishdir -= vec3::Y;
        }
        if self.input.move_right {
            wishdir += vec3::X;
        }
        if self.input.move_left {
            wishdir -= vec3::X;
        }
        if self.input.move_up {
            wishdir += vec3::Z;
        }
        if self.input.move_down {
            wishdir -= vec3::Z;
        }
        wishdir = wishdir.normalize_or_zero();
        wishdir *= Self::PLAYER_UNITS_PER_SECOND * SECONDS_PER_TICK;

        self.camera.change_camera_position(wishdir);

        //////////////
        // Rotation //
        //////////////

        // (Δyaw, Δpitch)
        let mut wishlook = vec2::ZERO;

        if self.input.rotate_left {
            wishlook += vec2::X;
        }
        if self.input.rotate_right {
            wishlook -= vec2::X;
        }
        if self.input.rotate_up {
            wishlook += vec2::Y;
        }
        if self.input.rotate_down {
            wishlook -= vec2::Y;
        }

        wishlook = wishlook.normalize_or_zero();
        wishlook *= Self::ANGLE_PER_SECOND * SECONDS_PER_TICK;

        // Without this vertical sens feels too fast
        wishlook.y /= Self::FOV_Y_RADIANS;

        self.camera.change_yaw(wishlook.x);
        self.camera.change_pitch(wishlook.y);
    }
}

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
                    _ => {
                        let result = self.input.process(code, event.state);
                        if matches!((result, event.state), (None, Pressed)) {
                            println!("warning: ignoring {code:?}");
                        }
                    }
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
        _: DeviceId,
        event: DeviceEvent,
    ) {
        use DeviceEvent::*;
        match event {
            MouseMotion { delta: (_, _) } => {
                // MouseMotion events are not affect by Windows Mouse Settings
                // according to https://github.com/bevyengine/bevy/issues/1149
                // ...and they keep coming when you hit the edge of the screen.
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
