use js_sys::Date;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, KeyboardEvent, MouseEvent};

use super::renderer::{load_meshes, load_shaders, load_textures, render, RendererState};
use super::WorldState;

use glow::Context;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct App {
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

        let renderer = RendererState {
            resolution: (canvas.client_width(), canvas.client_height()),
            pixels_per_centimeter: window().unwrap().device_pixel_ratio(),
            meshes: load_meshes(&gl).expect("Failed to laod meshes"),
            shaders: load_shaders(&gl).expect("Failed to laod shaders"),
            textures: load_textures(&gl).expect("Failed to load textures"),
        };

        Self {
            canvas,
            renderer,
            gl,
        }
    }

    pub fn animation_frame(&mut self) {
        update_resolution(&self.canvas, &mut self.renderer);

        let time = (Date::new_0().get_time() / 1000.0) as f32;
        render(&self.gl, &self.renderer, &WorldState { time });
    }

    pub fn keydown_event(&mut self, _event: KeyboardEvent) {
        // self.app.set_key_state(event.which(), true);
    }
    pub fn keyup_event(&mut self, _event: KeyboardEvent) {
        // self.app.set_key_state(event.which(), false);
    }

    pub fn mouse_event(&mut self, _event: MouseEvent) {}
}

fn update_resolution(canvas: &HtmlCanvasElement, state: &mut RendererState) {
    // This is a somewhat hacky version.
    // For a proper approach see
    // https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio

    let client_width = canvas.client_width();
    let client_height = canvas.client_height();
    let canvas_width = canvas.width() as i32;
    let canvas_height = canvas.height() as i32;

    let pixel_ratio = window().unwrap().device_pixel_ratio();

    let target_pixels_wide = (client_width as f64 * pixel_ratio) as i32;
    let target_pixels_high = (client_height as f64 * pixel_ratio) as i32;

    if canvas_width != target_pixels_wide || canvas_height != target_pixels_high {
        canvas.set_width(target_pixels_wide as u32);
        canvas.set_height(target_pixels_high as u32);
    }

    state.resolution.0 = target_pixels_wide;
    state.resolution.1 = target_pixels_high;

    // The pixel ratio is in terms of CSS magic-pixels which are already
    // HiDPI adjusted and are always an effective 96DPI.
    let pixels_per_centimeter = pixel_ratio * 96.0 / 2.54;
    state.pixels_per_centimeter = pixels_per_centimeter;
}
