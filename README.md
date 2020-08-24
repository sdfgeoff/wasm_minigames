# WASM Games From Scratch

![build status](https://travis-ci.org/sdfgeoff/wasm_minigames.svg?branch=master)

I got interested in shader toy a little while back, and enjoyed the ease
of creating shader games: you type and it happens. I wanted to bring the
same experience to normal game development, so decided to create games in WASM
using Rust. They're easy to make, easy to share.

This book documents my progress as I learn how to do so. The book (and
games made along the way) are hosted up at:

https://sdfgeoff.github.io/wasm_minigames/


## Creating a page with WASM on it:

1. Create a folder containing a Cargo.toml file inside the src directory. This can be done by copy/pasting an existing one.
2. Make sure the package name in the Cargo.toml file matches the directory name
3. Insert a page in the book with src/SUMMARY.md. I suggest putting `.md` files inside the cargo directory to keep them together
4. In a `md` file, reference the WASM using:

```
<!-- Basic: -->
<canvas id="swoop_ship_motion_physics"></canvas>

<!-- Passing an argument string to the WASM: -->
<canvas id="swoop_ship_motion_physics" options="some random stuff"></canvas>


<!-- Make sure the id is unique. You can do this with: -->
<canvas id="swoop_ship_motion_physics-1" options="some random stuff"></canvas>
<canvas id="swoop_ship_motion_physics-2" options="other random stuff"></canvas>
<canvas id="swoop_ship_motion_physics-3" options="the final one"></canvas>
<!-- The id is split on the '-' and the first part used to identify the WASM to load -->
```

## Developing fast.

Typing `make` builds all the examples and the whole book, so can be quite slow
(It seems that wasm-pack doesn't check that nothing has changed). So you can
instead type `make specific_example`. However, the book is then not up to
date, so instead you have to use `specific_example/pkg/index.html` to test the WASM

You can also use `make specific_example DEBUG=1` to do non-release builds for testing.
