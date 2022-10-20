use std::ops::{Sub, Mul, Add, Div, MulAssign, DivAssign, AddAssign, SubAssign};



#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub fg_default: bool,
    pub bg_default: bool,
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color{r, g, b, a: 1.0, fg_default: false, bg_default: false}
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color{r, g, b, a, fg_default: false, bg_default: false}
    }

    pub fn rgb_255(r: u8, g: u8, b: u8) -> Color {
        Color{r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0, fg_default: false, bg_default: false}
    }

    pub fn rgba_255(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color{r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: a as f32 / 255.0, fg_default: false, bg_default: false}
    }

    pub fn as_255(&self) -> Color {
        Color {
            r: self.r * 255.0,
            g: self.g * 255.0,
            b: self.b * 255.0,
            a: self.a * 255.0,
            fg_default: self.fg_default,
            bg_default: self.bg_default,
        }
    }

    pub fn clamp(&mut self, min_val: f32, max_val: f32) {
        if self.r > max_val {
            self.r = max_val;
        };
        if self.g > max_val {
            self.g = max_val;
        };
        if self.b > max_val {
            self.b = max_val;
        };
        if self.r < min_val {
            self.r = min_val;
        };
        if self.g < min_val {
            self.g = min_val;
        };
        if self.b < min_val {
            self.b = min_val;
        };
    }

    pub fn clamp_u8(&mut self, min_val: u8, max_val: u8) {
        let mut temp = self.as_255();

        temp.r = std::cmp::max(min_val, std::cmp::min(temp.r.round() as u8, max_val)) as f32;
        temp.g = std::cmp::max(min_val, std::cmp::min(temp.g.round() as u8, max_val)) as f32;
        temp.b = std::cmp::max(min_val, std::cmp::min(temp.b.round() as u8, max_val)) as f32;

        self.r = temp.r / 255.0;
        self.g = temp.g / 255.0;
        self.b = temp.b / 255.0;
    }

    pub fn gamma_corrected(&self) -> Color {
        let val_r = 1.0 - 0.3086;
        let val_g = 1.0 - 0.6094;
        let val_b = 1.0 - 0.0820;

        Color {
            r: self.r * val_r,
            g: self.g * val_g,
            b: self.b * val_b,
            a: self.a,
            fg_default: self.fg_default,
            bg_default: self.bg_default,
        }
    }

    pub fn white() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0, fg_default: false, bg_default: false }
    }

    pub fn black() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0, fg_default: false, bg_default: false }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}




impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
            fg_default: self.fg_default,
            bg_default: self.bg_default,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
            fg_default: self.fg_default,
            bg_default: self.bg_default,
        }
    }
}
impl Mul<u8> for Color {
    type Output = Color;
    fn mul(self, rhs: u8) -> Color {
        Color {
            r: self.r * rhs as f32,
            g: self.g * rhs as f32,
            b: self.b * rhs as f32,
            a: self.a,
            fg_default: self.fg_default,
            bg_default: self.bg_default,
        }
    }
}
impl MulAssign for Color {
    fn mul_assign(self: &mut Color, rhs: Color) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}
impl MulAssign<f32> for Color {
    fn mul_assign(self: &mut Color, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}
impl Div for Color {
    type Output = Color;
    fn div(self, rhs: Color) -> Color {
        Color {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
            a: self.a / rhs.a,
            fg_default: self.fg_default,
            bg_default: self.bg_default,
        }
    }
}
impl Div<f32> for Color {
    type Output = Color;
    fn div(self, rhs: f32) -> Color {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a,
            fg_default: self.fg_default,
            bg_default: self.bg_default,
        }
    }
}
impl DivAssign for Color {
    fn div_assign(self: &mut Color, rhs: Color) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}
impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
            fg_default: self.fg_default,
            bg_default: self.bg_default,
        }
    }
}
impl AddAssign for Color {
    fn add_assign(self: &mut Color, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
impl Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Color {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
            fg_default: self.fg_default,
            bg_default: self.bg_default,
        }
    }
}
impl SubAssign for Color {
    fn sub_assign(self: &mut Color, rhs: Color) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}