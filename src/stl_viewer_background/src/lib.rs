use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Function;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use web_sys::{window, Event, HtmlCanvasElement, KeyEvent, MouseEvent};

mod app;
mod geometry;
mod shader;
mod shader_stl;
mod shader_background;
mod stl;
mod background;
mod texture;
mod textures;
mod camera;

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// This struct will be accessible from JS as a JS object that can be
// created using `new Core()`
#[wasm_bindgen]
pub struct Core {
    app: Rc<RefCell<app::App>>,
    canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: String) -> Self {
        log(&format!("WASM Started for canvas {}", canvas_id));

        let selector = format!("#{}", canvas_id);

        let window = window().unwrap();
        let document = window.document().unwrap();
        let element = document
            .query_selector(&selector)
            .expect("Call failed")
            .expect("No element with selector");

        element.set_class_name("loaded");

        let canvas: HtmlCanvasElement = element.dyn_into().expect("Not a canvas");

        let app = Rc::new(RefCell::new(app::App::new(canvas.clone())));

        Self { app, canvas }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        log("App Started");
        let window = window().unwrap();

        {
            // Animation Frame
            let callback = Rc::new(RefCell::new(None));

            let anim_app = self.app.clone();
            let anim_window = window.clone();
            let anim_callback = callback.clone();

            *callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                anim_app.borrow_mut().animation_frame();
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
            let anim_app1 = self.app.clone();
            let anim_app2 = self.app.clone();
            let anim_app3 = self.app.clone();

            let mouse_move = Closure::wrap(Box::new(move |event: MouseEvent| {
                anim_app1.borrow_mut().mouse_move(event);
            }) as Box<dyn FnMut(_)>);
            let mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
                anim_app2.borrow_mut().mouse_up(event);
            }) as Box<dyn FnMut(_)>);
            let mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
                anim_app3.borrow_mut().mouse_down(event);
            }) as Box<dyn FnMut(_)>);

            let mouse_move_ref = mouse_move.as_ref().unchecked_ref();
            let mouse_up_ref = mouse_up.as_ref().unchecked_ref();
            let mouse_down_ref = mouse_down.as_ref().unchecked_ref();
            
            self.canvas
                .add_event_listener_with_callback("mousedown", mouse_down_ref)
                .unwrap();
            self.canvas
                .add_event_listener_with_callback("mouseup", mouse_up_ref)
                .unwrap();
            self.canvas
                .add_event_listener_with_callback("mousemove", mouse_move_ref)
                .unwrap();
            self.canvas
                .add_event_listener_with_callback("mouseleave", mouse_up_ref)
                .unwrap();

            mouse_move.forget();
            mouse_up.forget();
            mouse_down.forget();
        }

        {
            // keyboard events
            self.canvas.set_tab_index(1); // Canvas elements ignore key events unless they have a tab index
            let anim_app = self.app.clone();

            let callback = Closure::wrap(Box::new(move |event: KeyEvent| {
                let e: Event = event.clone().dyn_into().unwrap();
                e.stop_propagation();
                e.prevent_default();

                anim_app.borrow_mut().key_event(event);
            }) as Box<dyn FnMut(_)>);

            self.canvas
                .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
                .unwrap();
            self.canvas
                .add_event_listener_with_callback("keyup", callback.as_ref().unchecked_ref())
                .unwrap();

            callback.forget();
        }
    }
}

fn make_callback(closure: &Closure<dyn FnMut()>) -> &Function {
    closure.as_ref().unchecked_ref()
}
