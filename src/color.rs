use std::ops::{Mul, Add};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn to_hex(&self) -> u32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        (r << 16) | (g << 8) | b
    }

    pub fn is_black(&self) -> bool {
        self.r == 0.0 && self.g == 0.0 && self.b == 0.0
    }

    // Linear interpolation between two colors
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
        }
    }

    // New blend mode methods
    pub fn blend_normal(&self, blend: &Color) -> Color {
        if blend.is_black() { *self } else { *blend }
    }

    pub fn blend_multiply(&self, blend: &Color) -> Color {
        Color::new(
            self.r * blend.r / 255.0,
            self.g * blend.g / 255.0,
            self.b * blend.b / 255.0,
        )
    }

    pub fn blend_add(&self, blend: &Color) -> Color {
        Color::new(
            (self.r + blend.r).min(255.0),
            (self.g + blend.g).min(255.0),
            (self.b + blend.b).min(255.0),
        )
    }

    pub fn blend_subtract(&self, blend: &Color) -> Color {
        Color::new(
            self.r - blend.r.max(0.0),
            self.g - blend.g.max(0.0),
            self.b - blend.b.max(0.0),
        )
    }

    pub fn blend_screen(&self, blend: &Color) -> Color {
        Color::new(
            255.0 - ((255.0 - self.r) * (255.0 - blend.r) / 255.0),
            255.0 - ((255.0 - self.g) * (255.0 - blend.g) / 255.0),
            255.0 - ((255.0 - self.b) * (255.0 - blend.b) / 255.0),
        )
    }
}

// Implementar multiplicación de Color por un escalar (f32)
impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

// Implementar multiplicación de Color por otro Color (componente por componente)
impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

// Implementar la suma de dos colores
impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}
