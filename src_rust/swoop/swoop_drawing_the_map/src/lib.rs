use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Function;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use web_sys::{window, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent};

mod app;
mod map_sprite;
mod shader;
mod ship_sprite;
mod texture;
mod transform;

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
    pub fn new(canvas: HtmlCanvasElement, options: String) -> Self {
        log(&format!(
            "WASM Started for canvas '{}' with options '{}'",
            canvas.id(), options
        ));

        canvas.set_class_name("loaded");
        let app = Rc::new(RefCell::new(app::App::new(canvas.clone(), options)));

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
            let anim_app = self.app.clone();

            let callback = Closure::wrap(Box::new(move |event: MouseEvent| {
                anim_app.borrow_mut().mouse_event(event);
            }) as Box<dyn FnMut(_)>);

            let callback_ref = callback.as_ref().unchecked_ref();
            self.canvas
                .add_event_listener_with_callback("mousedown", callback_ref)
                .unwrap();
            self.canvas
                .add_event_listener_with_callback("mouseup", callback_ref)
                .unwrap();
            self.canvas
                .add_event_listener_with_callback("mousemove", callback_ref)
                .unwrap();
            self.canvas
                .add_event_listener_with_callback("mouseenter", callback_ref)
                .unwrap();
            self.canvas
                .add_event_listener_with_callback("mouseleave", callback_ref)
                .unwrap();
            self.canvas
                .add_event_listener_with_callback("mouseover", callback_ref)
                .unwrap();

            callback.forget();
        }

        {
            // keyboard events
            self.canvas.set_tab_index(1); // Canvas elements ignore key events unless they have a tab index
            let anim_app = self.app.clone();

            let callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
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
