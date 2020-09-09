use glium::uniforms::{AsUniformValue, UniformValue};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(f32, f32, f32, f32);

impl Color {
    pub fn red() -> Self {
        Self(1.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Self {
        Self(0.0, 1.0, 0.0, 1.0)
    }

    pub fn blue() -> Self {
        Self(0.0, 0.0, 1.0, 0.0)
    }

    pub fn white() -> Self {
        Self(1.0, 1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self(0.0, 0.0, 0.0, 1.0)
    }

    pub fn r(&self) -> f32 {
        self.0
    }

    pub fn g(&self) -> f32 {
        self.1
    }

    pub fn b(&self) -> f32 {
        self.2
    }

    pub fn a(&self) -> f32 {
        self.3
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::white()
    }
}

impl From<f32> for Color {
    fn from(color: f32) -> Self {
        Self(color, color, color, 1.0)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(rgb: (f32, f32, f32)) -> Self {
        Self(rgb.0, rgb.1, rgb.2, 1.0)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(rgba: (f32, f32, f32, f32)) -> Self {
        Self(rgba.0, rgba.1, rgba.2, rgba.3)
    }
}

impl From<[f32; 3]> for Color {
    fn from(rgb: [f32; 3]) -> Self {
        Self(rgb[0], rgb[1], rgb[2], 1.0)
    }
}

impl From<[f32; 4]> for Color {
    fn from(rgba: [f32; 4]) -> Self {
        Self(rgba[0], rgba[1], rgba[2], rgba[3])
    }
}

impl Into<(f32, f32, f32)> for Color {
    fn into(self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }
}

impl Into<(f32, f32, f32, f32)> for Color {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.0, self.1, self.2, self.3)
    }
}

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        [self.0, self.1, self.2]
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

impl AsUniformValue for Color {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec4([self.0, self.1, self.2, self.3])
    }
}
