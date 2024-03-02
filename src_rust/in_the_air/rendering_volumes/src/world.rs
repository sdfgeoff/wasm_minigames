use glam::Mat4;

pub struct Camera {
    pub transform: Mat4,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

pub struct Vehicle {
    pub transform: Mat4,
}

pub struct WorldState {
    pub time: f64,
    pub time_since_start: f32,
    pub camera: Camera,
    pub vehicles: Vec<Vehicle>,
}