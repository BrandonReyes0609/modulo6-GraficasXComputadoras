// Archivo: vertex_shader.rs
use crate::vertex::vertex::Vertex;
use crate::uniforms::Uniforms;
use nalgebra_glm::{Vec3, Vec4, Mat4};

pub fn vertex_shader(vertex: &Vertex, transform_matrix: &Mat4, viewport_matrix: &Mat4) -> Vertex {
    // Step 1: Apply the combined model, view, and projection transformation
    let clip_pos = transform_matrix * Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

    // Step 2: Perform perspective division to get normalized device coordinates (NDC)
    let ndc_pos = if clip_pos.w != 0.0 {
        Vec3::new(
            clip_pos.x / clip_pos.w,
            clip_pos.y / clip_pos.w,
            clip_pos.z / clip_pos.w,
        )
    } else {
        Vec3::new(clip_pos.x, clip_pos.y, clip_pos.z)
    };

    // Step 3: Apply viewport transformation to get screen coordinates
    let screen_pos = viewport_matrix * Vec4::new(ndc_pos.x, ndc_pos.y, ndc_pos.z, 1.0);

    // Create and return the transformed vertex
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_pos.x, screen_pos.y, screen_pos.z),
        transformed_normal: vertex.transformed_normal,
    }
}
