use super::*;
use image::{Pixel, Rgba};
use std::ops::{Mul,Add,Sub};

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba {
            0: [self.r, self.g, self.b, self.a],
        }
    }
}


pub struct PointInfo {
    pub distance : f32,
    pub normal : Vector3<f32>
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<PointInfo>;
    fn get_color(&self) -> Color;
    fn get_position(&self) -> Vector3<f32>;
    fn get_albedo(&self) -> f32;
}

pub enum Colors {
    BLUE,
    RED,
    GREEN,
    CYAN,
    YELLOW,
    MAGENTA,
    WHITE,
    GREY,
    SKYBLUE,
}

impl Colors {
    pub fn value(&self) -> Color {
        match self {
            Colors::BLUE => Color::new(0, 0, 255, 255),
            Colors::RED => Color::new(255, 0, 0, 255),
            Colors::GREEN => Color::new(0, 255, 0, 255),
            Colors::CYAN => Color::new(0, 255, 255, 255),
            Colors::MAGENTA => Color::new(255, 0, 255, 255),
            Colors::YELLOW => Color::new(255, 255, 0, 255),
            Colors::WHITE => Color::new(255, 255, 255, 255),
            Colors::GREY => Color::new(50, 50, 50, 255),
            Colors::SKYBLUE => Color::new(135,206,235,255),
            _ => Color::new(0, 0, 0, 255),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, value : f32) -> Color {
        Color {
            r : ((self.r as f32) * value) as u8,
            g : ((self.g as f32) * value) as u8,
            b : ((self.b as f32) * value) as u8,
            a : ((self.a as f32) * value) as u8,
        }
    }
}
impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, value : Color) -> Color {
        Color {
            r : self.r.checked_mul(value.r).unwrap_or(255),
            g : self.g.checked_mul(value.g).unwrap_or(255),
            b : self.b.checked_mul(value.b).unwrap_or(255),
            a : self.a.checked_mul(value.r).unwrap_or(255),
        }
    }
}
impl Add<Color> for Color {
    type Output = Color;
    fn add(self, value : Color) -> Color {
        Color {
            r : self.r.checked_add(value.r).unwrap_or(255),
            g : self.g.checked_add(value.g).unwrap_or(255),
            b : self.b.checked_add(value.b).unwrap_or(255),
            a : self.a.checked_add(value.r).unwrap_or(255),
        }
    }
}
impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, value : Color) -> Color {
        Color {
            r : self.r.checked_sub(value.r).unwrap_or(0),
            g : self.g.checked_sub(value.g).unwrap_or(0),
            b : self.b.checked_sub(value.b).unwrap_or(0),
            a : self.a.checked_sub(value.r).unwrap_or(0),
        }
    }
}