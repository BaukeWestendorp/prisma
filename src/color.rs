#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

#[allow(dead_code)]
impl Color {
    pub fn rgb(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn black() -> Self {
        Self::rgb(0.0, 0.0, 0.0, 1.0)
    }

    pub fn gray() -> Self {
        Self::rgb(0.5, 0.5, 0.5, 1.0)
    }

    pub fn white() -> Self {
        Self::rgb(1.0, 1.0, 1.0, 1.0)
    }

    pub fn red() -> Self {
        Self::rgb(1.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Self {
        Self::rgb(0.0, 1.0, 0.0, 1.0)
    }

    pub fn blue() -> Self {
        Self::rgb(0.0, 0.0, 1.0, 1.0)
    }

    pub fn transparent() -> Self {
        Self::rgb(0.0, 0.0, 0.0, 1.0)
    }

    pub fn blend_with(&mut self, other: &Color) {
        let lerp = |a, b, t| t * (b - a) + a;
        self.red = lerp(self.red, other.red, other.alpha);
        self.green = lerp(self.green, other.green, other.alpha);
        self.blue = lerp(self.blue, other.blue, other.alpha);
        self.alpha += other.alpha;
    }

    pub fn as_bytes(&self) -> [u8; 3] {
        let to_byte = |value: f32| -> u8 { (value.clamp(0.0, 1.0) * 255.0) as u8 };
        [to_byte(self.red), to_byte(self.green), to_byte(self.blue)]
    }
}
