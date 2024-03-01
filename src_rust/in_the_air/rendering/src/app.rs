use js_sys::Date;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, KeyboardEvent, MouseEvent};

use super::renderer::{
    load_framebuffers, load_shader_programs, load_textures, render, resize_buffers, RendererState,
};
use super::resources::StaticResources;
use super::world::{Camera, Vehicle, WorldState};
use glam::{EulerRot, Mat4, Quat, Vec3};

use glow::Context;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn debug_log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

pub struct App {
    world: WorldState,
    canvas: HtmlCanvasElement,
    renderer: RendererState,
    gl: glow::Context,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement, _options: String) -> Self {
        log!("[OK] Got App");
        let (gl, _shader_version) = {
            let webgl2_context = canvas
                .get_context("webgl2")
                .expect("Failed to get context 1")
                .expect("Failed to get context 2")
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .expect("Failed to get context 3");
            log!("[OK] Got Context");

            // Grab various extensions....
            let _float_texture_ext = webgl2_context.get_extension("OES_texture_float");
            let _float_texture_ext = webgl2_context.get_extension("EXT_color_buffer_float");

            #[cfg(target_arch = "wasm32")]
            let gl = Context::from_webgl2_context(webgl2_context);
            #[cfg(not(target_arch = "wasm32"))]
            let gl = unimplemented!();

            (gl, "#version 300 es")
        };
        log!("[OK] Got GL");

        let target_resolution = calculate_resolution(&canvas);

        let static_resources = StaticResources::load(&gl);
        let shader_programs =
            load_shader_programs(&gl, &static_resources).expect("Failed to load shaders");

        let textures = load_textures(&gl, &target_resolution).expect("Failed to load textures");
        let framebuffers = load_framebuffers(&gl, &textures).expect("Failed to load Fraimbuffers");

        let renderer = RendererState {
            resolution: target_resolution,
            pixels_per_centimeter: window().unwrap().device_pixel_ratio(),
            static_resources,
            shader_programs,
            textures: textures,
            framebuffers: framebuffers,
        };

        let world = WorldState {
            time: Date::new_0().get_time() / 1000.0,
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
            ],
        };

        Self {
            world,
            canvas,
            renderer,
            gl,
        }
    }

    pub fn animation_frame(&mut self) {
        update_resolution(&self.gl, &self.canvas, &mut self.renderer);

        let time = Date::new_0().get_time() / 1000.0;
        let delta = (self.world.time - time).abs() as f32;
        self.world.time = time;
        let time_since_start = self.world.time_since_start + delta;
        self.world.time_since_start = time_since_start;

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

        render(&self.gl, &self.renderer, &self.world);
    }

    pub fn keydown_event(&mut self, _event: KeyboardEvent) {
        // self.app.set_key_state(event.which(), true);
    }
    pub fn keyup_event(&mut self, _event: KeyboardEvent) {
        // self.app.set_key_state(event.which(), false);
    }

    pub fn mouse_event(&mut self, _event: MouseEvent) {}
}

fn update_resolution(gl: &Context, canvas: &HtmlCanvasElement, state: &mut RendererState) {
    // This is a somewhat hacky version.
    // For a proper approach see
    // https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio
    let canvas_width = canvas.width() as i32;
    let canvas_height = canvas.height() as i32;

    let target_resolution = calculate_resolution(canvas);

    if canvas_width != target_resolution[0] || canvas_height != target_resolution[1] {
        log!("Resizing To {:?}", target_resolution);
        canvas.set_width(target_resolution[0] as u32);
        canvas.set_height(target_resolution[1] as u32);

        resize_buffers(gl, state, &target_resolution);

        state.resolution = target_resolution;

        // The pixel ratio is in terms of CSS magic-pixels which are already
        // HiDPI adjusted and are always an effective 96DPI.
        let pixel_ratio = window().unwrap().device_pixel_ratio();

        let pixels_per_centimeter = pixel_ratio * 96.0 / 2.54;
        state.pixels_per_centimeter = pixels_per_centimeter;
    }
}

fn calculate_resolution(canvas: &HtmlCanvasElement) -> [i32; 2] {
    let client_width = canvas.client_width();
    let client_height = canvas.client_height();

    let pixel_ratio = window().unwrap().device_pixel_ratio();
    [
        (client_width as f64 * pixel_ratio) as i32,
        (client_height as f64 * pixel_ratio) as i32,
    ]
}
