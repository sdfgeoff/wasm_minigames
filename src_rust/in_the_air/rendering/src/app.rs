use js_sys::Date;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, KeyboardEvent, MouseEvent};

use super::renderer::{
    load_framebuffers, load_shader_programs, load_textures, render, resize_buffers,
    RendererState,
};
use super::world::{
    WorldState,
    Camera,
    Vehicle
};
use glam::{Vec3, Mat4, Quat, EulerRot};
use super::resources::StaticResources;

use glow::Context;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
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

            let gl = Context::from_webgl2_context(webgl2_context);
            (gl, "#version 300 es")
        };
        log!("[OK] Got GL");

        let target_resolution = calculate_resolution(&canvas);

        let static_resources = StaticResources::load(&gl);
        let shader_programs = load_shader_programs(&gl, &static_resources).expect("Failed to load shaders");

        let textures = load_textures(&gl, &target_resolution)
            .expect("Failed to load textures");
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
            time: 0.0,
            camera: Camera {
                fov: 90.0,
                near: 0.1,
                far: 1000.0,
                transform: Mat4::from_rotation_translation(
                    Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                )
            },

            vehicles: vec![
                Vehicle {
                    transform: Mat4::from_rotation_translation(
                        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
                        Vec3::new(0.0, 0.0, 10.0),
                    )
                },
                Vehicle {
                    transform: Mat4::from_rotation_translation(
                        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
                        Vec3::new(0.0, 0.0, 10.0),
                    )
                }
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
        self.world.time = time;

        self.world.vehicles[0].transform = Mat4::from_rotation_translation(
            Quat::from_euler(EulerRot::XYZ, 0.0, time.sin() as f32, 0.0),
            Vec3::new(0.0, 0.0, 10.0),
        );
        self.world.vehicles[1].transform = Mat4::from_rotation_translation(
            Quat::from_euler(EulerRot::XYZ, time.cos() as f32, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 10.0),
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
        (client_height as f64 * pixel_ratio) as i32
    ] 
}