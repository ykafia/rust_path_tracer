use super::*;

#[derive(Copy,Clone)]
pub struct Material<'a> {
    albedo : f32,
    emissive : Emissive<'a>
}

#[derive(Copy,Clone)]
pub struct Texture<'a> {
    pixels : &'a [Rgba<u8>]
}

#[derive(Copy,Clone)]
pub struct TexCoord {
    x : usize,
    y : usize
}

#[derive(Copy,Clone)]
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