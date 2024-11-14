// Archivo: fragment.rs
pub mod fragment {
    use crate::color::Color;

    #[derive(Clone, Copy, Debug)]
    pub struct Fragment {
        pub position_x: f32,
        pub position_y: f32,
        pub color: Color,
        pub depth: f32,
    }

    impl Fragment {
        pub fn new(position_x: f32, position_y: f32, color: Color, depth: f32) -> Self {
            Self {
                position_x,
                position_y,
                color,
                depth,
            }
        }
    }
}

