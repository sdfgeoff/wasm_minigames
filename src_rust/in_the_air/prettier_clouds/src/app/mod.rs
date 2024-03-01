use glam::{EulerRot, Mat4, Quat, Vec3};

use glow::Context;

pub mod attributes;
pub mod framebuffer;
pub mod keyboard;
pub mod mesh;
pub mod mesh_loader;
pub mod renderer;
pub mod resources;
pub mod shader;
pub mod shader_program;
pub mod texture;
pub mod world;

use renderer::{
    load_framebuffers, load_shader_programs, load_textures, render, resize_buffers, RendererState,
};
use resources::{ResourceError, StaticResources};
use shader::ShaderError;
use world::{Camera, Vehicle, WorldState};

use super::debug_log;

pub struct App {
    world: WorldState,
    renderer: RendererState,
    gl: glow::Context,
    keyboard: keyboard::Keyboard,
}

impl App {
    pub fn new(
        gl: Context,
        _options: String,
        time: f64,
        screen_resolution: [i32; 2],
        pixels_per_centimeter: f64,
    ) -> Self {
        debug_log("[OK] Got App");

        let static_resources = match StaticResources::load(&gl) {
            Ok(resources) => resources,
            Err(ResourceError::ShaderError(ShaderError::ShaderCompileError {
                shader_type: _,
                compiler_output,
                shader_text,
            })) => {
                let lines = shader_text.split('\n');
                for (line_id, line_text) in lines.enumerate() {
                    debug_log(&format!("{:4} | {}", line_id + 1, line_text));
                }
                panic!("Shader Compile Error: {}", compiler_output);
            }
            Err(err) => {
                debug_log(&format!("[ERR] Failed to load static resources: {:?}", err));
                panic!("Failed to load static resources");
            }
        };

        let shader_programs =
            load_shader_programs(&gl, &static_resources).expect("Failed to load shaders");

        let textures = load_textures(&gl, &screen_resolution).expect("Failed to load textures");
        let framebuffers = load_framebuffers(&gl, &textures).expect("Failed to load Fraimbuffers");

        let renderer = RendererState {
            resolution: screen_resolution,
            pixels_per_centimeter: pixels_per_centimeter,
            static_resources,
            shader_programs,
            textures: textures,
            framebuffers: framebuffers,
        };

        let world = WorldState {
            time,
            time_since_start: 0.0,

            camera: Camera {
                fov: 3.14159 / 3.0,
                near: 0.1,
                far: 1000.0,
                transform: Mat4::from_translation(Vec3::new(0.0, 0.0, 40.0)),
            },

            vehicles: vec![
                Vehicle {
                    transform: Mat4::from_rotation_translation(
                        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
                        Vec3::new(0.0, 0.0, 0.0),
                    ),
                },
                Vehicle {
                    transform: Mat4::from_rotation_translation(
                        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
                        Vec3::new(0.0, 0.0, 0.0),
                    ),
                },
                Vehicle {
                    transform: Mat4::from_rotation_translation(
                        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
                        Vec3::new(0.0, 0.0, 0.0),
                    ),
                },
                Vehicle {
                    transform: Mat4::from_rotation_translation(
                        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
                        Vec3::new(0.0, 0.0, 0.0),
                    ),
                },
            ],
        };

        Self {
            world,
            renderer,
            gl,
            keyboard: keyboard::Keyboard::new(),
        }
    }

    pub fn update_resolution(&mut self, resolution: [i32; 2], pixels_per_centimeter: f64) {
        debug_log(&format!("Resizing To {:?}", resolution));
        resize_buffers(&self.gl, &self.renderer, &resolution);

        self.renderer.pixels_per_centimeter = pixels_per_centimeter;
        self.renderer.resolution = resolution;
    }

    pub fn animation_frame(&mut self, time: f64) {
        let delta = (self.world.time - time).abs() as f32;
        self.world.time = time;
        let time_since_start = self.world.time_since_start + delta;
        self.world.time_since_start = time_since_start;

        fly_camera(&mut self.world.camera, &self.keyboard, delta);

        self.world.vehicles[0].transform = Mat4::from_rotation_translation(
            Quat::from_euler(EulerRot::XYZ, 0.0, time_since_start.sin(), 0.0),
            Vec3::new(0.0, 10.0, 0.0),
        );
        self.world.vehicles[1].transform = Mat4::from_rotation_translation(
            Quat::from_euler(EulerRot::XYZ, time_since_start.sin(), 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        );

        self.world.vehicles[2].transform = Mat4::from_rotation_translation(
            Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                0.0,
                time_since_start + (time_since_start * 2.0).sin(),
            ),
            Vec3::new(time_since_start.cos() * 2.0, time_since_start.sin(), 0.0) * 20.0,
        );

        self.world.vehicles[1].transform = self.world.camera.transform
            * Mat4::from_translation(Vec3::new(0.0, -5.0, -20.0))
            * Mat4::from_rotation_translation(
                Quat::from_euler(EulerRot::XYZ, -1.5, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            );

        render(&self.gl, &self.renderer, &self.world);
    }

    pub fn key_event(&mut self, key: keyboard::KeyCode, is_down: bool) {
        self.keyboard.set_key_state(key, is_down);
    }

    pub fn mouse_event(&mut self) {}
}

fn fly_camera(camera: &mut Camera, key_state: &keyboard::Keyboard, delta: f32) {
    let mut translation = Vec3::new(0.0, 0.0, 0.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);

    if key_state.is_key_pressed(keyboard::KeyCode::W) {
        translation[2] -= 1.0;
    }
    if key_state.is_key_pressed(keyboard::KeyCode::S) {
        translation[2] += 1.0;
    }
    if key_state.is_key_pressed(keyboard::KeyCode::A) {
        translation[0] -= 1.0;
    }
    if key_state.is_key_pressed(keyboard::KeyCode::D) {
        translation[0] += 1.0;
    }
    if key_state.is_key_pressed(keyboard::KeyCode::R) {
        translation[1] += 1.0;
    }
    if key_state.is_key_pressed(keyboard::KeyCode::F) {
        translation[1] -= 1.0;
    }

    if key_state.is_key_pressed(keyboard::KeyCode::Up) {
        rotation[0] += 1.0;
    }
    if key_state.is_key_pressed(keyboard::KeyCode::Down) {
        rotation[0] -= 1.0;
    }
    if key_state.is_key_pressed(keyboard::KeyCode::Left) {
        rotation[1] += 1.0;
    }
    if key_state.is_key_pressed(keyboard::KeyCode::Right) {
        rotation[1] -= 1.0;
    }
    if key_state.is_key_pressed(keyboard::KeyCode::Q) {
        rotation[2] += 1.0;
    }
    if key_state.is_key_pressed(keyboard::KeyCode::E) {
        rotation[2] -= 1.0;
    }

    let translation_local = translation * delta * 100.0;
    let rotation_local = rotation * delta * 2.0;

    let translation_global = camera.transform.transform_vector3(translation_local);
    let rotation_global = camera.transform.transform_vector3(rotation_local);
    camera.transform = Mat4::from_translation(translation_global) * camera.transform;

    //camera.transform = Mat4::from_quat( Quat::from_euler(EulerRot::XYZ, rotation_global[0], rotation_global[1], rotation_global[2])) * camera.transform;

    camera.transform = Mat4::from_translation(camera.transform.w_axis.truncate())
        * Mat4::from_quat(Quat::from_euler(
            EulerRot::XYZ,
            rotation_global[0],
            rotation_global[1],
            rotation_global[2],
        ))
        * Mat4::from_translation(-camera.transform.w_axis.truncate())
        * camera.transform;
}
