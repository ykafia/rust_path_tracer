use super::*;
use image::{Pixel, Rgba};
use std::ops::{Mul,Add,Sub};
use std::convert::TryInto;

#[derive(Clone,Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba {
            0: [
                ((self.r * 255.0) as u32).try_into().unwrap_or(255), 
                ((self.g * 255.0) as u32).try_into().unwrap_or(255), 
                ((self.b * 255.0) as u32).try_into().unwrap_or(255), 
                ((self.a * 255.0) as u32).try_into().unwrap_or(255)
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


pub struct PointInfo {
    pub distance : f32,
    pub normal : Vector3<f32>,
    pub intersection : Vector3<f32>
}

#[derive(Copy,Clone)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}
impl Intersectable for Element {
    fn simple_intersect(&self, ray: &Ray) -> bool {
        match *self {
            Element::Sphere(ref s) => s.simple_intersect(ray),
            Element::Plane(ref p) => p.simple_intersect(ray)
        }

    }
    fn intersect(&self, ray: &Ray) -> Option<PointInfo> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray)
        }
    }
    fn get_color(&self) -> Color {
        match *self {
            Element::Sphere(ref s) => s.get_color(),
            Element::Plane(ref p) => p.get_color()
        }
    }
    fn get_albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.get_albedo(),
            Element::Plane(ref p) => p.get_albedo()
        }
    }
    fn get_position(&self) -> Vector3<f32> {
        match *self {
            Element::Sphere(ref s) => s.get_position(),
            Element::Plane(ref p) => p.get_position()
        }
    }
}


pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<PointInfo>;
    fn simple_intersect(&self, ray:&Ray) -> bool;
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
            BLACK => Color::new(0.0,0.0,0.0, 1.0),
            _ => Color::new(0.0,0.0,0.0, 1.0)
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
