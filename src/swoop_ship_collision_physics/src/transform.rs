type Vec2 = (f32, f32);

/// A non-generic transform in 2D. Only supports rotations translations
/// and a uniform scaling.
#[derive(Debug)]
pub struct Transform2d {
    pub x: f32,
    pub y: f32,
    pub rot: f32,
    pub scale: f32,
}

impl Transform2d {
    pub fn new(x: f32, y: f32, rot: f32, scale: f32) -> Self {
        Self { x, y, rot, scale }
    }
    pub fn to_mat3_array(&self) -> [f32; 9] {
        let c = f32::cos(self.rot) * self.scale;
        let s = f32::sin(self.rot) * self.scale;

        [c, -s, self.x, s, c, self.y, 0.0, 0.0, 1.0]
    }
}

pub fn vect_between(trans1: &Transform2d, trans2: &Transform2d) -> Vec2 {
    (
        trans1.x - trans2.x,
        trans1.y - trans2.y,
    )
}

pub fn length(vect: Vec2) -> f32 {
    f32::sqrt(vect.0 * vect.0 + vect.1 * vect.1)
}

pub fn normalize(vect: Vec2) -> Vec2 {
    let len = length(vect);
    (
        vect.0 / len,
        vect.1 / len,
    )
}
