use super::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Material {
    pub albedo : f32,
    pub emissive : Surface,
    pub reflectivity : Option<f32>
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
pub enum Surface {
    Color(Color),
    Texture(DynamicImage)
}


impl Surface {
    pub fn color(&self, coord : TexCoord) -> Color {
        match self {
            Surface::Color(c) => c.clone(),
            Surface::Texture(t) => {
                let color = Color::from(t.get_pixel(
                    (coord.x*t.width() as f32) as u32,
                    (coord.y*t.height() as f32) as u32
                ));
                color
            }
        }
    }
}
