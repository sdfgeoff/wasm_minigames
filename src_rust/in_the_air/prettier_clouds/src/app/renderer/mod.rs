use super::framebuffer::FrameBuffer;
use super::shader_program::ShaderProgram;
use super::texture::Texture;
use glam::Mat4;
use glow::{Context, HasContext};

use super::resources::StaticResources;
use super::world::{Camera, WorldState};

mod pipeline;
mod setup;
pub use setup::{load_framebuffers, load_shader_programs, load_textures};

pub struct ShaderPrograms {
    model: ShaderProgram,
    volume_and_light: ShaderProgram,
    passthrough: ShaderProgram,
}

pub struct Textures {
    buffer_color: Texture,
    buffer_material: Texture,
    buffer_geometry: Texture,
    buffer_depth: Texture,

    buffer_display: Texture,
}

pub struct FrameBuffers {
    gbuffer: FrameBuffer,
    display_buffer: FrameBuffer,
}

pub struct RendererState {
    pub resolution: [i32; 2],
    pub pixels_per_centimeter: f64,
    pub shader_programs: ShaderPrograms,
    pub textures: Textures,
    pub framebuffers: FrameBuffers,
    pub static_resources: StaticResources,
}

pub struct CameraMatrices {
    world_to_camera: Mat4,
    camera_to_world: Mat4,
    camera_to_screen: Mat4,
}

pub fn camera_to_matrices(camera: &Camera, resolution: &[i32; 2]) -> CameraMatrices {
    let camera_to_world = camera.transform;
    let world_to_camera = camera_to_world.inverse();
    let camera_to_screen = Mat4::perspective_rh_gl(
        camera.fov,
        resolution[0] as f32 / resolution[1] as f32,
        camera.near,
        camera.far,
    );
    CameraMatrices {
        world_to_camera,
        camera_to_world,
        camera_to_screen,
    }
}

pub fn render(gl: &Context, renderer_state: &RendererState, world_state: &WorldState) {
    let camera_matrices = camera_to_matrices(&world_state.camera, &renderer_state.resolution);

    unsafe {
        gl.enable(glow::DEPTH_TEST);
    }

    pipeline::render_gbuffer(gl, renderer_state, world_state, &camera_matrices);
    pipeline::render_volume_and_lighting(gl, renderer_state, world_state, &camera_matrices);
    pipeline::render_to_display(gl, renderer_state, world_state);
}

/// Configures the resolution of all of the textures used in the deferred geometry pipeline.
/// This ensure they match up with each other and the outside world.
pub fn resize_buffers(gl: &Context, renderer_state: &RendererState, resolution: &[i32; 2]) {
    renderer_state
        .textures
        .buffer_color
        .resize_render_target(gl, resolution);
    renderer_state
        .textures
        .buffer_material
        .resize_render_target(gl, resolution);
    renderer_state
        .textures
        .buffer_geometry
        .resize_render_target(gl, resolution);
    renderer_state
        .textures
        .buffer_depth
        .resize_render_target(gl, resolution);

    renderer_state
        .textures
        .buffer_display
        .resize_render_target(gl, resolution);
}
