use crate::fragment::Fragment;
use crate::uniforms::Uniforms;
use crate::color::Color;
use std::f32::consts::PI;

// Static pattern shader
fn static_pattern_shader(fragment: &Fragment) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let pattern = ((x * 10.0).sin() * (y * 10.0).sin()).abs();

    let r = (pattern * 255.0) as u8;
    let g = ((1.0 - pattern) * 255.0) as u8;
    let b = 128;

    Color::new(r as f32, g as f32, b as f32)
}

// Moving circles shader
fn moving_circles_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let time = uniforms.time as f32 * 0.05;
    let circle1_x = (time.sin() * 0.4 + 0.5) % 1.0;
    let circle2_x = (time.cos() * 0.4 + 0.5) % 1.0;

    let dist1 = ((x - circle1_x).powi(2) + (y - 0.3).powi(2)).sqrt();
    let dist2 = ((x - circle2_x).powi(2) + (y - 0.7).powi(2)).sqrt();

    let circle_size = 0.1;
    let circle1 = if dist1 < circle_size { 1.0f32 } else { 0.0f32 };
    let circle2 = if dist2 < circle_size { 1.0f32 } else { 0.0f32 };

    let circle_intensity = (circle1 + circle2).min(1.0f32);

    Color::new(
        (circle_intensity * 255.0) as f32,
        (circle_intensity * 255.0) as f32,
        (circle_intensity * 255.0) as f32,
    )
}

// Combined shader
pub fn combined_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_color = static_pattern_shader(fragment);
    let circle_color = moving_circles_shader(fragment, uniforms);

    // Combine shaders: use circle color if it's not black, otherwise use base color
    if !circle_color.is_black() {
        circle_color * fragment.intensity
    } else {
        base_color * fragment.intensity
    }
}

// Combined shader with blend modes
pub fn combined_blend_shader(fragment: &Fragment, blend_mode: &str) -> Color {
    let base_color = static_pattern_shader(fragment);
    let circle_color = moving_circles_shader(fragment, &Uniforms::new());

    let combined_color = match blend_mode {
        "normal" => base_color.blend_normal(&circle_color),
        "multiply" => base_color.blend_multiply(&circle_color),
        "add" => base_color.blend_add(&circle_color),
        "screen" => base_color.blend_screen(&circle_color),
        _ => base_color,
    };

    combined_color * fragment.intensity
}
