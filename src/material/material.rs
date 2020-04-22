use super::*;

#[derive(Copy,Clone,Debug)]
pub struct Material<'a> {
    pub albedo : f32,
    pub emissive : Emissive<'a>
}

#[derive(Copy,Clone,Debug)]
pub struct Texture<'a> {
    pub pixels : &'a [Rgba<u8>],
    pub width : usize,
    pub height : usize
}

impl<'a> Texture<'a> {
    pub fn get_pixel(&self, x : usize, y : usize) -> Rgba<u8> {
        if y>self.width {
            panic!("{} is too big, width is {}",y,self.width);
        }
        if x>self.height {
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

#[derive(Copy,Clone,Debug)]
pub enum Emissive<'a> {
    Color(Color),
    Texture(Texture<'a>)
}


impl<'a> Emissive<'a> {
    pub fn color(&self, coord : &TexCoord) -> Color {
        match *self {
            Emissive::Color(c) => c,
            Emissive::Texture(t) => {
                Color::from(t.get_pixel(
                    (coord.x * t.height as f32) as usize,
                    (coord.y * t.width as f32) as usize
                ))
            }
        }
    }
}