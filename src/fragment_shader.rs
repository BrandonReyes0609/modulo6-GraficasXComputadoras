// Archivo: fragment_shader.rs
use crate::fragment::Fragment; // Cambiar a una referencia directa
use crate::uniforms::Uniforms;
use crate::color::Color;

pub fn fragment_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    // Aplica la intensidad de fragmento al color base
    fragment.color * fragment.intensity
}
