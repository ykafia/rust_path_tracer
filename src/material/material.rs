use super::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Material {
    pub albedo : f32,
    pub emissive : Emissive
}

#[derive(Clone,Debug)]
pub struct Texture {
    pub pixels : Arc<Vec<Rgba<u8>>>,
    pub width : usize,
    pub height : usize
}

impl Texture {
    pub fn get_pixel(&self, x : usize, y : usize) -> Rgba<u8> {
        if y>=self.width {
            panic!("{} is too big, width is {}",y,self.width);
        }
        if x>=self.height {
            panic!("{} is too big, height is {}",x,self.height);
        }
        self.pixels[x*self.width + y]
    }
}

#[derive(Copy,Clone)]
pub struct TexCoord {
    pub x : f32,
    pub y : f32
}

#[derive(Clone)]
pub enum Emissive {
    Color(Color),
    Texture(DynamicImage)
}


impl Emissive {
    pub fn color(&self, coord : TexCoord) -> Color {
        match self {
            Emissive::Color(c) => c.clone(),
            Emissive::Texture(t) => {
                // println!("Texture : [{}-{}]",(coord.x*t.height as f32) as usize,(coord.y*t.width as f32) as usize);
                // println!("{:?}",t);
                let color = Color::from(t.get_pixel(
                    (coord.x*t.width() as f32) as u32,
                    (coord.y*t.height() as f32) as u32
                ));
                color
            }
        }
    }
}