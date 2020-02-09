use super::*;
use image::{Pixel, Rgba};

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

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn get_color(&self) -> Color;
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
            Colors::GREY => Color::new(155, 155, 155, 255),
            Colors::SKYBLUE => Color::new(135,206,235,255),
            _ => Color::new(0, 0, 0, 255),
        }
    }
}
