use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use super::transform::{length, normalize, Vec2};
// TODO: rewrite map to be easily portable

pub struct Map {
    pub sin_consts: [f32; 8],
    pub cos_consts: [f32; 8],
    pub track_base_radius: f32,
    pub track_width: f32,
}

impl Map {
    pub fn distance_field(&self, position: Vec2) -> f32 {
        let course = length(position);
        let angle = position.0.atan2(position.1);

        let mut track_radius = self.track_base_radius;
        for i in 0..8 {
            let omega = (i + 1) as f32;
            track_radius += f32::sin(angle * omega) * self.sin_consts[i];
            track_radius += f32::cos(angle * omega) * self.cos_consts[i];
        }

        let mut track_sdf = course - track_radius;
        track_sdf = f32::abs(track_sdf) - self.track_width;
        return track_sdf;
    }

    // Uses finite difference to approximate the normal. This isn't quite
    // the actual normal because the distance field isn't quite the distance
    // field.
    pub fn calc_normal(&self, position: Vec2) -> Vec2 {
        const DELTA: f32 = 0.01;
        let here = self.distance_field(position);
        let above = self.distance_field((position.0, position.1 + DELTA));
        let right = self.distance_field((position.0 + DELTA, position.1));

        let dx = right - here;
        let dy = above - here;

        return normalize((dx, dy));
    }
}
