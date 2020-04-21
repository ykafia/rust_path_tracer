use super::*;

#[derive(Copy,Clone,Debug)]
pub struct Material<'a> {
    pub albedo : f32,
    pub emissive : Emissive<'a>
}

#[derive(Copy,Clone,Debug)]
pub struct Texture<'a> {
    pub pixels : &'a [Rgba<u8>]
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
                Colors::YELLOW.value()
            }
        }
    }
}