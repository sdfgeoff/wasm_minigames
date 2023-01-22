use std::cell::RefCell;
use std::rc::Rc;

use glow::Context;
use js_sys::{Date, Function};

use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, KeyboardEvent, MouseEvent};

mod app;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn debug_log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

// This struct will be accessible from JS as a JS object that can be
// created using `new Core()`
#[wasm_bindgen]
pub struct Core {
    app: Rc<RefCell<app::App>>,
    canvas: Rc<RefCell<HtmlCanvasElement>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, options: String) -> Self {
        console_error_panic_hook::set_once();

        log!(
            "WASM Started for canvas '{}' with options '{}'",
            canvas.id(),
            options
        );

        canvas.set_class_name("loaded");

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

        let (target_resolution, pixels_per_centimeter) = calculate_resolution(&canvas);
        let time = Date::new_0().get_time() / 1000.0;

        let app = Rc::new(RefCell::new(app::App::new(
            gl,
            options,
            time,
            target_resolution,
            pixels_per_centimeter,
        )));
        let canvas = Rc::new(RefCell::new(canvas));

        Self { app, canvas }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        log!("App Started");
        let window = window().unwrap();

        {
            // Animation Frame
            let callback = Rc::new(RefCell::new(None));

            let anim_app = self.app.clone();
            let anim_window = window.clone();
            let anim_callback = callback.clone();
            let anim_canvas = self.canvas.clone();

            *callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                let mut app_ref = anim_app.borrow_mut();
                if let Some((target_resolution, pixels_per_centimeter)) =
                    check_update_resolution(&anim_canvas.borrow())
                {
                    app_ref.update_resolution(target_resolution, pixels_per_centimeter);
                }

                let time = Date::new_0().get_time() / 1000.0;

                app_ref.animation_frame(time);

                // Schedule ourself for another requestAnimationFrame callback.
                anim_window
                    .request_animation_frame(make_callback(
                        anim_callback.borrow().as_ref().unwrap(),
                    ))
                    .unwrap();
            }) as Box<dyn FnMut()>));
            window
                .request_animation_frame(make_callback(callback.borrow().as_ref().unwrap()))
                .unwrap();
        }

        {
            // Mouse events
            let anim_app = self.app.clone();
            let canvas = self.canvas.borrow();

            let callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
                anim_app.borrow_mut().mouse_event();
            }) as Box<dyn FnMut(_)>);

            let callback_ref = callback.as_ref().unchecked_ref();
            canvas
                .add_event_listener_with_callback("mousedown", callback_ref)
                .unwrap();
            canvas
                .add_event_listener_with_callback("mouseup", callback_ref)
                .unwrap();
            canvas
                .add_event_listener_with_callback("mousemove", callback_ref)
                .unwrap();
            canvas
                .add_event_listener_with_callback("mouseenter", callback_ref)
                .unwrap();
            canvas
                .add_event_listener_with_callback("mouseleave", callback_ref)
                .unwrap();
            canvas
                .add_event_listener_with_callback("mouseover", callback_ref)
                .unwrap();

            callback.forget();
        }

        {
            let canvas = self.canvas.borrow();

            // keyboard events
            canvas.set_tab_index(1); // Canvas elements ignore key events unless they have a tab index
            let anim_app1 = self.app.clone();
            let anim_app2 = self.app.clone();

            let keydown_callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                event.stop_propagation();
                event.prevent_default();

                if let Some(keycode) = keyboard::KeyCode::from_js_code(&event.code()) {
                    anim_app1.borrow_mut().key_event(keycode, true);
                }
            }) as Box<dyn FnMut(_)>);

            let keyup_callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                event.stop_propagation();
                event.prevent_default();

                if let Some(keycode) = keyboard::KeyCode::from_js_code(&event.code()) {
                    anim_app2.borrow_mut().key_event(keycode, false);
                }
            }) as Box<dyn FnMut(_)>);

            canvas
                .add_event_listener_with_callback(
                    "keydown",
                    keydown_callback.as_ref().unchecked_ref(),
                )
                .unwrap();

            canvas
                .add_event_listener_with_callback("keyup", keyup_callback.as_ref().unchecked_ref())
                .unwrap();

            keydown_callback.forget();
            keyup_callback.forget();
        }
    }
}

fn make_callback(closure: &Closure<dyn FnMut()>) -> &Function {
    closure.as_ref().unchecked_ref()
}

fn check_update_resolution(canvas: &HtmlCanvasElement) -> Option<([i32; 2], f64)> {
    // This is a somewhat hacky version.
    // For a proper approach see
    // https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio
    let canvas_width = canvas.width() as i32;
    let canvas_height = canvas.height() as i32;

    let (target_resolution, pixels_per_centimeter) = calculate_resolution(canvas);

    if canvas_width != target_resolution[0] || canvas_height != target_resolution[1] {
        canvas.set_width(target_resolution[0] as u32);
        canvas.set_height(target_resolution[1] as u32);

        Some((target_resolution, pixels_per_centimeter))
    } else {
        None
    }
}

fn calculate_resolution(canvas: &HtmlCanvasElement) -> ([i32; 2], f64) {
    let client_width = canvas.client_width();
    let client_height = canvas.client_height();

    let pixel_ratio = window().unwrap().device_pixel_ratio();

    // The pixel ratio is in terms of CSS magic-pixels which are already
    // HiDPI adjusted and are always an effective 96DPI.

    let pixels_per_centimeter = pixel_ratio * 96.0 / 2.54;

    (
        [
            (client_width as f64 * pixel_ratio) as i32,
            (client_height as f64 * pixel_ratio) as i32,
        ],
        pixels_per_centimeter,
    )
}
