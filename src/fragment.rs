// Archivo: fragment.rs
use nalgebra_glm::{Vec3, Vec2};
use crate::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct Fragment {
    pub position: Vec2,
    pub vertex_position: Vec3, // Agregado para almacenar la posición interpolada del vértice
    pub color: Color,
    pub depth: f32,
    pub normal: Vec3,
    pub intensity: f32,
}

impl Fragment {
    pub fn new(position: Vec2, vertex_position: Vec3, color: Color, depth: f32, normal: Vec3, intensity: f32) -> Self {
        Fragment {
            position,
            vertex_position,
            color,
            depth,
            normal,
            intensity,
        }
    }
}
