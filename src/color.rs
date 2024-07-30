use std::ops;

use crate::ray::Ray;


#[derive(Debug, Clone, Copy)]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl ColorRGBA {
    
    pub fn new(r: f32, g: f32, b: f32) -> Option<Self> {
        if vec![r, g, b].into_iter().any(|x| x < 0.0 || x > 255.0) {
            None
        } else {
            Some(Self{r, g, b, a: 0.0})
        }
    }

    pub fn white_blue_blend_over_y(r: &Ray) -> Self {
        let unit = r.direction.normalise();
        let a = 0.5 * (unit.y + 1.0) as f32;
        let white = Self::new(1.0, 1.0, 1.0).unwrap();
        let blue = Self::new(0.5, 0.7, 1.0).unwrap();
        ((1.0 - a) * white + a * blue).unwrap()
    }
}

impl ops::Mul<f32> for ColorRGBA {
    type Output = ColorRGBA;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a
        }
    }
}

impl ops::Mul<ColorRGBA> for f32 {
    type Output = ColorRGBA;

    fn mul(self, rhs: ColorRGBA) -> Self::Output {
        rhs * self
    }
}

impl ops::Add<ColorRGBA> for ColorRGBA {
    type Output = Option<ColorRGBA>;

    fn add(self, rhs: ColorRGBA) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}
