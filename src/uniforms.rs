use nalgebra_glm::{Mat4, Vec3, translate, rotate_x, rotate_y, rotate_z, scale, look_at, perspective};
use std::f32::consts::PI;

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: u32, // Frame counter for animations
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            model_matrix: Mat4::identity(),
            view_matrix: Mat4::identity(),
            projection_matrix: Mat4::identity(),
            viewport_matrix: Mat4::identity(),
            time: 0,
        }
    }
}

// Hacer las funciones públicas para que puedan ser usadas en otros módulos
pub fn create_model_matrix(translation: Vec3, scale_factor: f32, rotation: Vec3) -> Mat4 {
    let mut model_matrix = Mat4::identity();
    model_matrix = translate(&model_matrix, &translation);
    model_matrix = rotate_x(&model_matrix, rotation.x);
    model_matrix = rotate_y(&model_matrix, rotation.y);
    model_matrix = rotate_z(&model_matrix, rotation.z);
    model_matrix = scale(&model_matrix, &Vec3::new(scale_factor, scale_factor, scale_factor));
    model_matrix
}

pub fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

pub fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

pub fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}
