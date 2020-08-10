# Building and loading WASM

WASM builds have some requirements. For example, you need to define the library
type as `cdylib`.

It's probably easiest to look at a working Cargo.toml:
```toml
{{#include ./Cargo.toml}}
```

Rust has a great tool called [wasm-pack](https://github.com/rustwasm/wasm-pack) which makes the process of building
and deploying WASM code nice and simple. It's designed to work with bundlers,
but to avoid the gigabyte of dependencies that webpack pulls in, I decided to
go for the simplest output type: "web".

My invocation of `wasm-pack` is:

```shell
wasm-pack build --out-dir $(OUT_DIR) --target web --dev
# OR
wasm-pack build --out-dir $(OUT_DIR) --target web --release
```

When invoked, this will create a bunch of files: `core_bg.wasm`, `core.js`, `core_bg.d.ts`, `core.d.ts` and `package.json`.
The only files we need are `core_bg.wasm` (the actual webassembly) and `core.js` (code that loads the WASM).

Now you need to load it from HTML/js. For all the examples in this book, loading
is an invocation of the function:

```javascript
{{#include ../../custom.js}}
```

using an element like:

```html
<canvas id="building_and_loading_wasm"></canvas>
```

A very simple rust webassembly program looks like:

```rust
{{#include ./src/lib.rs}}
```

All up this creates:

<canvas id="building_and_loading_wasm"></canvas>

You'll notice when you click on it plays a loading animation. That's done in
in CSS. Normally this would get cancelled from inside the WASM binary, but
this example doesn't.

To check if this example is working, you have to look at the browser console.
You should see something like:

```
Loading ../games/trivial/core.js
WASM Started for canvas trivial
App Started
```

The first message comes from the javascript. The other two come from the WASM.
The message will only appear once, as the javascript prevents the WAS loading
twice.
