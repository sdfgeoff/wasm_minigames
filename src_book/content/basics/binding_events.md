# Binding Events

To make a game you need to have input from the user such as the keyboard and
mouse. You also need to have a mainloop (or some other way to update what the
user see's). In a browser, these are emitted as
[events](https://developer.mozilla.org/en-US/docs/Web/Events) and updating
your program can be done using
[requestAnimationFrame()](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame)

First lets deal with `requestAnimationFrame`. There's an example [on the
wasm-bindgen site](https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html)
which in theory makes this a copy-paste exercise.

In practice I didn't manage to get `request_animation_frame` to be able to
invoke a function on the `Core` struct. The issue is that you have to have
multiple references to the Core struct (so you can invoke
request_animation_frame again) so you need to put it in a `Rc`. However you
can't return the `Rc` from the constructor. As a result, I decided that the
`Core` struct would create an `App` struct. The `App` struct looks like:

```rust
struct App {
    canvas: HtmlCanvasElement,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        Self { canvas }
    }

    pub fn animation_frame(&mut self) {
        log("Animation Frame")
    }

    pub fn mouse_event(&mut self, event: MouseEvent) {
        log(&format!("Mouse Event {:?}", event));
    }
    pub fn key_event(&mut self, event: KeyEvent) {
        log(&format!("Key Event {:?}", event));
    }
}
```
Where Event is a custom enum that I'll populate with the events that the
application cares about (eg Mouse/Keyboard/Resize).

Then binding the animation_frame looks like:
```rust
fn make_callback(closure: &Closure<dyn FnMut()>) -> &Function {
    return closure.as_ref().unchecked_ref()
}

<< snip >>

let callback = Rc::new(RefCell::new(None));

let anim_app = self.app.clone();
let anim_window = window.clone();
let anim_callback = callback.clone();

*callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    anim_app.borrow_mut().animation_frame();
    // Schedule ourself for another requestAnimationFrame callback.
    anim_window
        .request_animation_frame(make_callback(anim_callback.borrow().as_ref().unwrap()));
}) as Box<dyn FnMut()>));
window.request_animation_frame(make_callback(callback.borrow().as_ref().unwrap()));
```
I will happily admit I'm 100% sure about everything going on in here. I haven't
figured out trait objects yet.

Fortunately, handling the other key and mouse is a bit easier because they aren't recursive:
```rust
let anim_app = self.app.clone();

let callback = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    anim_app.borrow_mut().mouse_event(event);
}) as Box<dyn FnMut(_)>);

let callback_ref = callback.as_ref().unchecked_ref();
self.canvas.add_event_listener_with_callback("mousedown", callback_ref).unwrap();
self.canvas.add_event_listener_with_callback("mouseup", callback_ref).unwrap();
self.canvas.add_event_listener_with_callback("mousemove", callback_ref).unwrap();
self.canvas.add_event_listener_with_callback("mouseenter", callback_ref).unwrap();
self.canvas.add_event_listener_with_callback("mouseleave", callback_ref).unwrap();
self.canvas.add_event_listener_with_callback("mouseover", callback_ref).unwrap();

callback.forget();
```

There were a bunch of gotchas with key events. For some reason, key events only
fire for canvas' when they have a tabindex and the canvas is focused. I wasted
a good hour or two on this thinking that mdbook was gobbling the input with its
document-level event handler....

Another gotcha with key events is that we need to stop the browser respoding to
them. This is easy enough with `e.stop_propagation()` and `e.prevent_default()`
which both prevent other handlers on the page and the browser from seeing the
event.

<canvas id="basics/binding_events"></canvas>

Once again there's nothing visible, but if you check the console you'll see all
the events reported by the WASM

