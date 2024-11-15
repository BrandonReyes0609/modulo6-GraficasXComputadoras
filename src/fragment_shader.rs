// Archivo: fragment_shader.rs
use crate::fragment::Fragment;
use crate::uniforms::Uniforms;
use crate::color::Color;

pub fn fragment_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    // Define stripe colors
    let colors = [
        Color::new(1.0, 0.0, 0.0),    // Red
        Color::new(0.0, 1.0, 0.0),    // Green
        Color::new(0.0, 0.0, 1.0),    // Blue
        Color::new(1.0, 1.0, 0.0),    // Yellow
    ];

    // Define stripe width
    let stripe_width = 0.1;

    // Use the y-coordinate of the transformed position for stripe calculation
    let stripe_coord = fragment.vertex_position.y;

    // Calculate which stripe this fragment belongs to
    let stripe_float = (stripe_coord / stripe_width).abs();
    let stripe_index = (stripe_float as usize) % colors.len();
    let next_index = (stripe_index + 1) % colors.len();

    // Calculate interpolation factor
    let t = stripe_float.fract();

    // Interpolate between current and next color
    colors[stripe_index].lerp(&colors[next_index], t) * fragment.intensity
}
