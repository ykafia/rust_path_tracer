use euc::{buffer::Buffer2d, rasterizer, DepthStrategy, Pipeline,Target};
use image::RgbImage;
use minifb::Window;
use vek::{Mat4, Vec2, Vec3, Vec4};
use nalgebra::{Vector4,Vector3,Vector2,Matrix4};
use euc::interpolate::Interpolate;

#[derive(Clone)]
pub struct Vec2f(Vector2<f32>);

impl Vec2f {
    pub fn x(&self) -> f32 {
        self.0.x
    }
    pub fn y(&self) -> f32 {
        self.0.y
    }
    
}

//TODO : implement interpolation
impl Interpolate for Vec2f {
    fn lerp2(val_1 : Self,val_2 : Self,i_1 : f32, i_2 : f32) -> Vec2f{
        Vec2f(Vector2::new(0f32,0f32))
    }
    fn lerp3(val_1 : Self,val_2 : Self, val_3 : Self,i_1 : f32, i_2 : f32, i_3 : f32) -> Vec2f{
        Vec2f(Vector2::new(0f32,0f32))
    }
}



pub struct Triangle;

pub struct Quad<'a> {
    mvp: &'a Matrix4<f32>,
    positions: &'a [Vector4<f32>],
    uvs: &'a [Vector2<f32>],
    texture: &'a RgbImage,
}

impl Pipeline for Triangle {
    type Vertex = [f32; 4];
    type VsOut = ();
    type Pixel = u32;

    // Vertex shader
    // - Returns the 3D vertex location, and the VsOut value to be passed to the fragment shader
    #[inline(always)]
    fn vert(&self, pos: &[f32; 4]) -> ([f32; 4], Self::VsOut) {
        (*pos, ())
    }

    // Specify the depth buffer strategy used for each draw call
    #[inline(always)]
    fn get_depth_strategy(&self) -> DepthStrategy {
        DepthStrategy::None
    }

    // Fragment shader
    // - Returns (in this case) a u32
    #[inline(always)]
    fn frag(&self, _: &Self::VsOut) -> Self::Pixel {
        let bytes = [255, 0, 0, 255]; // Red

        (bytes[2] as u32) << 0
            | (bytes[1] as u32) << 8
            | (bytes[0] as u32) << 16
            | (bytes[3] as u32) << 24
    }
}

fn matrix_to_array(m :Vector4<f32>) -> [f32;4] {
    [
        m.x,m.y,m.z,m.w
    ]
}

impl<'a> Pipeline for Quad<'a> {
    type Vertex = usize;
    type VsOut = Vec2f;
    type Pixel = u32;

    #[inline]
    fn vert(&self, v_index: &Self::Vertex) -> ([f32; 4], Self::VsOut) {
        (
            matrix_to_array(*self.mvp * self.positions[*v_index]),
            Vec2f(self.uvs[*v_index]),
        )
    }

    #[inline]
    fn frag(&self, v_uv: &Self::VsOut) -> Self::Pixel {
        // Convert interpolated uv coordinate to texture coordinate
        let (width, height) = (self.texture.width() as f32, self.texture.height() as f32);
        let x = f32::min(f32::max(0.0, v_uv.x() * width), width - 1.0);
        let y = f32::min(f32::max(0.0, v_uv.y() * height), height - 1.0);
        // Lookup pixel and convert to appropriate format
        let rgb = self.texture.get_pixel(x as u32, y as u32);
        255 << 24 | (rgb[0] as u32) << 16 | (rgb[1] as u32) << 8 | (rgb[2] as u32) << 0
    }
}

const W: usize = 640;
const H: usize = 480;

pub fn window() {
    let mut color = Buffer2d::new([W, H], 0);

    Triangle.draw::<rasterizer::Triangles<(f32,)>, _>(
        &[
            [-1.0, -1.0, 0.0, 1.0],
            [1.0, -1.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
        ],
        &mut color,
        None,
    );

    let mut win = minifb::Window::new("Triangle", W, H, minifb::WindowOptions::default()).unwrap();
    while win.is_open() {
        win.update_with_buffer(color.as_ref(), W, H).unwrap();
    }
}