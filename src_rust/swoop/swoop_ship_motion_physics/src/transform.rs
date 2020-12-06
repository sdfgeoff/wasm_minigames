/// A non-generic transform in 2D. Only supports rotations translations
/// and a uniform scaling.
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
