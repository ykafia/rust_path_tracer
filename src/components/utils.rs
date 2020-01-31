use super::*;
use image::{Pixel,Rgba};


#[derive(Clone)]
pub struct Color{
    pub r : u8,
    pub g : u8,
    pub b : u8,
    pub a : u8
}


impl Color {
    pub fn new(red : u8, green : u8, blue : u8, alpha : u8) -> Color{
        Color {
            r : red,
            g : green,
            b : blue,
            a : alpha
        }
    }
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba{
            0 : [self.r,self.g,self.b,self.a]
        }
    }
}


pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}
