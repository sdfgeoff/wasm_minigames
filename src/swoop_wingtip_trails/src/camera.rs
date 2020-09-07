use super::transform::{length, Transform2d, Vec2};

/// How far ahead of the target to position the camera.
const PREDICT_FACTOR: f32 = 0.65;
/// Smooths the camera motion to avoid rapid camera motion if the
/// velocity or position suddenly change.
/// Notes:
///  - this applies to the zoom as well
///  - this applies to the motion after the prediction, so a higher smoothing
///    will normaly also need a higher PREDICT_FACTOR
const SMOOTHING: f32 = 0.4;

/// What zoom level to have when the target is not moving
const ZOOM_BASE: f32 = 1.0;
/// How much the camera zooms out per unit velocity.
const ZOOM_FACTOR: f32 = 0.125;
/// What the zoom is set to after calling "reset"
const RESET_ZOOM: f32 = 10.0;

/// Represents a camera following/predicting a moving target. The
/// faster the target is moving, the further ahead the camera will
/// predict and the more zoomed out it will be.
pub struct Camera {
    /// Current position of the camera
    position: Vec2,

    /// Current zoom of the camera
    zoom: f32,

    /// Set to the current position of the object to track
    pub target_position: Vec2,

    /// Set to the current velocity of the object to track
    pub target_velocity: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            zoom: 1.0,
            target_position: (0.0, 0.0),
            target_velocity: (0.0, 0.0),
        }
    }

    pub fn reset(&mut self) {
        self.position = (0.0, 0.0);
        self.zoom = RESET_ZOOM;
        self.target_position = (0.0, 0.0);
        self.target_velocity = (0.0, 0.0);
    }

    /// Update the position of the camera, moving it towards the target
    /// position.
    pub fn update(&mut self, dt: f32) {
        let ideal_position = (
            self.target_position.0 + self.target_velocity.0 * PREDICT_FACTOR,
            self.target_position.1 + self.target_velocity.1 * PREDICT_FACTOR,
        );
        let velocity = length(&self.target_velocity);
        let ideal_zoom = ZOOM_BASE + velocity * ZOOM_FACTOR;

        let zoom_err = self.zoom - ideal_zoom;
        let pos_err = (
            self.position.0 - ideal_position.0,
            self.position.1 - ideal_position.1,
        );

        self.zoom -= zoom_err * dt / SMOOTHING;

        self.position.0 -= pos_err.0 * dt / SMOOTHING;
        self.position.1 -= pos_err.1 * dt / SMOOTHING;
    }

    /// Converts the camera position into an array that can be used in
    /// a shader.
    pub fn get_camera_matrix(&self, base_resolution: f32) -> [f32; 9] {
        Transform2d::new(
            self.position.0,
            self.position.1,
            0.0,
            1.0 / base_resolution * self.zoom,
        )
        .to_mat3_array()
    }
}
