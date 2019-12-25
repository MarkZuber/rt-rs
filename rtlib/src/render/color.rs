use std::iter::Sum;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

#[inline]
fn clamp_f32(val: f32, min: f32, max: f32) -> f32 {
    val.max(min).min(max)
}

#[inline]
fn de_nan_value(val: f32) -> f32 {
    if val.is_nan() || val.is_infinite() {
        0.0
    } else {
        val
    }
}

impl Color {
    pub fn zero() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn b(&self) -> f32 {
        self.b
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: clamp_f32(self.r, 0.0, 1.0),
            g: clamp_f32(self.g, 0.0, 1.0),
            b: clamp_f32(self.b, 0.0, 1.0),
        }
    }

    pub fn apply_gamma(&self) -> Color {
        Color {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }

    pub fn multiply_by_scalar(&self, scalar: f32) -> Color {
        Color {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }

    pub fn add(&self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }

    pub fn multiply(&self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }

    pub fn de_nan(&self) -> Color {
        Color {
            r: de_nan_value(self.r),
            g: de_nan_value(self.g),
            b: de_nan_value(self.b),
        }
    }
}

impl Sum for Color {
    fn sum<I>(iter: I) -> Color
    where
        I: Iterator<Item = Color>,
    {
        let mut curcol = Color::new(0.0, 0.0, 0.0);

        for i in iter {
            curcol = curcol.add(i.de_nan())
        }

        curcol
    }
}
