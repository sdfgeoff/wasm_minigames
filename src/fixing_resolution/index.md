# Fixing the Resolution

We have sucessfully rendered a triangle with a shader, however it's kind-of
blurry. This is because the resolution of the buffer that the webgl is rendering
does not match the resolution of the image in the browser.

We need to have some code that figures out what size the canvas output is, and
makes sure that the webgl has that as it's output resolution.

Easy enough:

```
    fn check_resize(&mut self) {
        let client_width = self.canvas.client_width();
        let client_height = self.canvas.client_height();
        let canvas_width = self.canvas.width() as i32;
        let canvas_height = self.canvas.height() as i32;
        
        if client_width != canvas_width || client_height != canvas_height {
            self.canvas.set_width(client_width as u32);
            self.canvas.set_height(client_height as u32);
            self.gl
                .viewport(0, 0, client_width, client_height);
            log(&format!("Resized to {}:{}", client_width, client_height));
        }
    }
```

I ran this inside the `updateAnimationFrame` loop. This probably isn't ideal
because it involves sending/receiving data bewteen WASM and JS lots of times
per second. Unfortunately there isn't an "onresize" event that works for
generic elements. This could be run just when the canvas initalizes, but then
it won't catch the user zooming.

<canvas id="fixing_resolution"></canvas>
