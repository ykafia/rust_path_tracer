use super::*;
use image::Rgba;
use std::ops::{Mul,Add,Sub};
use std::convert::TryInto;

#[derive(Clone,Copy,Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[allow(dead_code)]
impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }
    pub fn to_vec(&self) -> Vec<u8>{
        [
            ((self.r * 255.0) as u32).try_into().unwrap_or(255), 
            ((self.g * 255.0) as u32).try_into().unwrap_or(255), 
            ((self.b * 255.0) as u32).try_into().unwrap_or(255), 
            255
        ].to_vec()
    }
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba {
            0: [
                ((self.r * 255.0) as u32).try_into().unwrap_or(255), 
                ((self.g * 255.0) as u32).try_into().unwrap_or(255), 
                ((self.b * 255.0) as u32).try_into().unwrap_or(255), 
                255
            ],
        }
    }
    
    pub fn normalize(&mut self) {
        self.r = self.r.min(0.0).max(0.0);
        self.g = self.g.min(0.0).max(0.0);
        self.b = self.b.min(0.0).max(0.0);
        self.a = self.a.min(0.0).max(0.0);

    }
}



#[allow(dead_code)]
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
    BLACK
}

impl Colors {
    pub fn value(&self) -> Color {
        match self {
            Colors::BLUE => Color::new(0.0,0.0, 1.0, 1.0),
            Colors::RED => Color::new(1.0,0.0,0.0, 1.0),
            Colors::GREEN => Color::new(0.0, 1.0,0.0, 1.0),
            Colors::CYAN => Color::new(0.0, 1.0, 1.0, 1.0),
            Colors::MAGENTA => Color::new(1.0,0.0, 1.0, 1.0),
            Colors::YELLOW => Color::new(1.0, 1.0,0.0, 1.0),
            Colors::WHITE => Color::new(1.0, 1.0, 1.0, 1.0),
            Colors::GREY => Color::new(100.0/255.0, 100.0/255.0, 100.0/255.0, 1.0),
            Colors::SKYBLUE => Color::new(135.0/255.0,206.0/255.0,235.0/255.0,1.0),
            Colors::BLACK => Color::new(0.0,0.0,0.0, 1.0)
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, value : f32) -> Color {
        Color {
            r : self.r * value,
            g : self.g * value,
            b : self.b * value,
            a : self.a 
        }
    }
}
impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, value : Color) -> Color {
        Color {
            r : self.r * value.r,
            g : self.g * value.g,
            b : self.b * value.b,
            a : self.a 
        }
    }
}
impl Add<Color> for Color {
    type Output = Color;
    fn add(self, value : Color) -> Color {
        Color {
            r : self.r + value.r,
            g : self.g + value.g,
            b : self.b + value.b,
            a : self.a 
        }
    }
}
impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Color>>(iter: I) -> Self {
        let mut result = Colors::BLACK.value();
        for i in iter {
            result = result + i;
        }
        result
    }

}
impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, value : Color) -> Color {
        Color {
            r : self.r - value.r,
            g : self.g - value.g,
            b : self.b - value.b,
            a : self.a
        }
    }
}
