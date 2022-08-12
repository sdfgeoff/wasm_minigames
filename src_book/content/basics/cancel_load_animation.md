# Cancel the load animation

On this page, when clicking on a canvas, it sets the css style to "loading"
which creates a load animation. To stop the load animation, the webassembly
needs to change the classname of the canvas to "loaded" rather than "loading".
To do this we need to access the DOM.

You may have noticed that in the rust on the previous page the ID of the
canvas gets passed in. This allows us to find the element on the page.

In JS we would use `document.getElementById(id)`. For some reason this doesn't
exist in web-sys, so instead we can use `document.query_selector`:
```rust
let window = window().unwrap();
let document = window.document().unwrap();
let element = document
    .query_selector(&selector)
    .expect("Call failed")
    .expect("No element with selector");

element.set_class_name("loaded");
```
There's a fair bit of unwrap/expecting going on there, and it isn't ideal.
However, I'm not sure there really is any good way for the program to handle
the place where it's trying to draw not existing, so it will do for now.

To get this code to compile, you need a bunch of things:

In your Cargo.toml you need a bunch of features from the web-sys crate:
```toml
[dependencies.web-sys]
version = "0.3.4"
features = [
    "Document",
    "HtmlCanvasElement",
    "HtmlElement",
    "Window",
]
```
We're using those API's, so it all makes sense. There's also the `dyn_into`
thing, which is in `wasm_bindgen::JsCast;`. This allows casting from a generic
`HTMLElement` into a `HtmlCanvasElement`.

The result:

<canvas id="basics/cancel_load_animation"></canvas>

When you click it, it goes black - the contents of the `loaded` style
