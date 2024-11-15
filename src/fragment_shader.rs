use crate::fragment::Fragment;
use crate::uniforms::Uniforms;
use crate::color::Color;
use std::f32::consts::PI;

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Define a list of colors to cycle through
    let colors = [
        Color::new(255.0, 0.0, 0.0),    // Red
        Color::new(0.0, 255.0, 0.0),    // Green
        Color::new(0.0, 0.0, 255.0),    // Blue
        Color::new(255.0, 255.0, 0.0),  // Yellow
        Color::new(255.0, 0.0, 255.0),  // Magenta
        Color::new(0.0, 255.0, 255.0),  // Cyan
    ];

    // Define how many frames to show each color
    let frames_per_color = 100;

    // Calculate which color we should be showing
    let color_index = (uniforms.time / frames_per_color) as usize % colors.len();

    // Calculate how far we are into the transition to the next color
    let transition_progress = (uniforms.time % frames_per_color) as f32 / frames_per_color as f32;

    // Get the current color and the next color
    let current_color = colors[color_index];
    let next_color = colors[(color_index + 1) % colors.len()];

    // Interpolate between the current color and the next color
    current_color.lerp(&next_color, transition_progress) * fragment.intensity
}

// Shader to create moving horizontal lines
pub fn moving_stripes_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Define stripe colors
    let color1 = Color::new(255.0, 0.0, 0.0);  // Red
    let color2 = Color::new(0.0, 0.0, 255.0);  // Blue

    // Define stripe properties
    let stripe_width = 0.2;  // Width of each stripe
    let speed = 0.002;       // Speed of stripe movement

    // Calculate the y-coordinate for the moving effect
    let moving_y = fragment.vertex_position.y + uniforms.time as f32 * speed;

    // Create the stripe pattern
    let stripe_factor = ((moving_y / stripe_width) * PI).sin() * 0.5 + 0.5;

    // Interpolate between the two colors based on the stripe factor
    color1.lerp(&color2, stripe_factor) * fragment.intensity
}

// Shader to create moving dot effect
pub fn moving_dots_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Define colors
    let background_color = Color::new(250.0, 250.0, 250.0); // Light gray
    let dot_color = Color::new(255.0, 0.0, 0.0);            // Red

    // Define dot properties
    let dot_size = 0.1;
    let dot_spacing = 0.3;
    let speed = 0.001;

    // Calculate moving coordinates
    let moving_x = fragment.vertex_position.x + uniforms.time as f32 * speed;
    let moving_y = fragment.vertex_position.y - uniforms.time as f32 * speed * 0.5;

    // Create dot pattern using sine and cosine
    let pattern_x = ((moving_x / dot_spacing) * 2.0 * PI).cos();
    let pattern_y = ((moving_y / dot_spacing) * 2.0 * PI).cos();

    // Combine patterns to create dots
    let dot_pattern = (pattern_x * pattern_y).max(0.0);

    // Apply dot size
    let dot_factor = (dot_pattern - (1.0 - dot_size)).max(0.0) / dot_size;

    // Interpolate between background color and dot color
    background_color.lerp(&dot_color, dot_factor) * fragment.intensity
}
