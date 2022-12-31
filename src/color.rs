use std::ops;

#[must_use]
#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn from_u32(v: u32) -> Self {
        #[allow(clippy::erasing_op, clippy::identity_op)]
        Self::from_byte_array([
            (v >> (u8::BITS * 3) & 0xFF) as _,
            (v >> (u8::BITS * 2) & 0xFF) as _,
            (v >> (u8::BITS * 1) & 0xFF) as _,
            (v >> (u8::BITS * 0) & 0xFF) as _,
        ])
    }

    pub fn from_array([r, g, b, a]: [f32; 4]) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_byte_array([r, g, b, a]: [u8; 4]) -> Self {
        Self {
            r: r as f32 / 255.,
            g: g as f32 / 255.,
            b: b as f32 / 255.,
            a: a as f32 / 255.,
        }
    }

    pub fn into_byte_array(self) -> [u8; 4] {
        [
            (self.r * 255.) as _,
            (self.g * 255.) as _,
            (self.b * 255.) as _,
            (self.a * 255.) as _,
        ]
    }

    pub fn is_visible(self) -> bool {
        self.a > f32::EPSILON
    }

    pub fn is_transparent(self) -> bool {
        self.a < 1.
    }

    pub fn overlay(self, rhs: Self) -> Self {
        Self {
            r: lerp(self.r, rhs.r, rhs.a),
            g: lerp(self.g, rhs.g, rhs.a),
            b: lerp(self.b, rhs.b, rhs.a),
            a: self.a.max(rhs.a),
        }
    }

    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        Self {
            r: lerp(self.r, rhs.r, t),
            g: lerp(self.g, rhs.g, t),
            b: lerp(self.b, rhs.b, t),
            a: lerp(self.a, rhs.a, t),
        }
    }
}

fn lerp(x: f32, y: f32, t: f32) -> f32 {
    x + t * (y - x)
}

impl From<u32> for Color {
    fn from(v: u32) -> Self {
        Self::from_u32(v)
    }
}

impl From<[f32; 4]> for Color {
    fn from(v: [f32; 4]) -> Self {
        Self::from_array(v)
    }
}

impl From<[u8; 4]> for Color {
    fn from(v: [u8; 4]) -> Self {
        Self::from_byte_array(v)
    }
}

impl From<Color> for [u8; 4] {
    fn from(v: Color) -> Self {
        v.into_byte_array()
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}
