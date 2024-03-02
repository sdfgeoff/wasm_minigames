use super::{CameraMatrices, RendererState};
use crate::{shader_program::ShaderProgram, world::WorldState};
use glam::{Mat4, Vec4};
use glow::{Context, HasContext};

use crate::app::debug_log;

pub fn apply_camera_to_shader(gl: &Context, camera: &CameraMatrices, shader: &ShaderProgram) {
    unsafe {
        gl.uniform_matrix_4_f32_slice(
            shader.uniforms.get("camera_to_world"),
            false,
            &camera.camera_to_world.to_cols_array(),
        );
        gl.uniform_matrix_4_f32_slice(
            shader.uniforms.get("world_to_camera"),
            false,
            &camera.world_to_camera.to_cols_array(),
        );
        gl.uniform_matrix_4_f32_slice(
            shader.uniforms.get("camera_to_screen"),
            false,
            &camera.camera_to_screen.to_cols_array(),
        );
    }
}

pub fn render_gbuffer(
    gl: &Context,
    renderer_state: &RendererState,
    world_state: &WorldState,
    camera_matrices: &CameraMatrices,
) {
    // Render Opaque geometry to the G-buffer
    renderer_state.framebuffers.gbuffer.bind(gl);

    unsafe {
        gl.viewport(
            0,
            0,
            renderer_state.resolution[0],
            renderer_state.resolution[1],
        );
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }

    let active_shader_program = &renderer_state.shader_programs.model;
    let active_mesh = &renderer_state.static_resources.meshes.vehicle_med_res;
    active_shader_program.bind(gl);
    active_mesh.bind(gl, &active_shader_program.attributes);

    apply_camera_to_shader(gl, camera_matrices, active_shader_program);

    renderer_state
        .static_resources
        .textures
        .vehicle_albedo
        .bind_to_uniform(gl, 0, active_shader_program.uniforms.get("albedo_texture"));

    renderer_state
        .static_resources
        .textures
        .vehicle_roughness_metal
        .bind_to_uniform(
            gl,
            1,
            active_shader_program
                .uniforms
                .get("metallic_roughness_texture"),
        );

    for vehicle in world_state.vehicles.iter() {
        unsafe {
            gl.uniform_matrix_4_f32_slice(
                active_shader_program.uniforms.get("world_to_model"),
                false,
                &vehicle.transform.inverse().to_cols_array(),
            );

            gl.uniform_matrix_4_f32_slice(
                active_shader_program.uniforms.get("model_to_world"),
                false,
                &vehicle.transform.to_cols_array(),
            );
        }
        active_mesh.render(gl);
    }
}

pub fn render_volume_and_lighting(
    gl: &Context,
    renderer_state: &RendererState,
    _world_state: &WorldState,
    _camera_matrices: &CameraMatrices,
) {
    // Render our GBuffer to the Display Buffer
    renderer_state.framebuffers.display_buffer.bind(gl);

    unsafe {
        gl.viewport(
            0,
            0,
            renderer_state.resolution[0],
            renderer_state.resolution[1],
        );
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }

    let active_shader_program = &renderer_state.shader_programs.volume_and_light;
    let active_mesh = &renderer_state.static_resources.meshes.quad_quad;

    active_shader_program.bind(gl);
    active_mesh.bind(gl, &active_shader_program.attributes);

    renderer_state.textures.buffer_color.bind_to_uniform(
        gl,
        0,
        renderer_state
            .shader_programs
            .volume_and_light
            .uniforms
            .get("buffer_color"),
    );
    renderer_state.textures.buffer_geometry.bind_to_uniform(
        gl,
        1,
        renderer_state
            .shader_programs
            .volume_and_light
            .uniforms
            .get("buffer_geometry"),
    );
    renderer_state.textures.buffer_material.bind_to_uniform(
        gl,
        2,
        renderer_state
            .shader_programs
            .volume_and_light
            .uniforms
            .get("buffer_material"),
    );

    active_mesh.render(gl);
}

pub fn render_to_display(gl: &Context, renderer_state: &RendererState, _world_state: &WorldState) {
    // Forward the display buffer to the screen
    unsafe {
        gl.bind_framebuffer(glow::FRAMEBUFFER, None);

        gl.viewport(
            0,
            0,
            renderer_state.resolution[0],
            renderer_state.resolution[1],
        );

        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }

    let active_shader_program = &renderer_state.shader_programs.passthrough;
    let active_mesh = &renderer_state.static_resources.meshes.quad_quad;

    renderer_state.shader_programs.passthrough.bind(gl);
    active_mesh.bind(gl, &active_shader_program.attributes);

    renderer_state.textures.buffer_display.bind_to_uniform(
        gl,
        0,
        renderer_state
            .shader_programs
            .passthrough
            .uniforms
            .get("input_texture"),
    );

    active_mesh.render(gl);
}