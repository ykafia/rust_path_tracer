use super::*;

#[derive(Copy, Clone)]
pub struct DirectionalLight {
    pub direction: Vector3<f32>,
    pub color: Color,
    pub intensity: f32,
}

impl DirectionalLight {
    pub fn new(direction: Vector3<f32>, color: Color, intensity: f32) -> Self { Self { direction, color, intensity } }
}




#[derive(Copy, Clone)]
pub struct PointLight {
    pub position: Vector3<f32>,
    pub color: Color,
    pub intensity: f32,
}
#[derive(Copy, Clone)]
pub enum Light {
    DirectionalLight(DirectionalLight),
    PointLight(PointLight),
}

impl Light {
    pub fn get_direction(&self, element: &Element) -> Vector3<f32> {
        match *self {
            Light::DirectionalLight(ref d) => d.direction,
            Light::PointLight(ref p) => element.get_position() - p.position,
        }
    }
    pub fn get_color(&self) -> Color {
        match *self {
            Light::DirectionalLight(ref d) => d.color,
            Light::PointLight(ref p) => p.color,
        }
    }
    pub fn get_intensity(&self, intersection : Vector3<f32>) -> f32 {
        match *self {
            Light::DirectionalLight(ref d) => d.intensity,
            Light::PointLight(ref p) => {
                let diff = intersection - p.position;
                let (x, y, z) = (diff.x, diff.y, diff.z);
                (p.intensity / (x * x + y * y + z * z).sqrt())
            }
        }
    }
    #[allow(dead_code)]
    pub fn set_position(&mut self, pos : Vector3<f32>){
        match *self {
            Light::PointLight(ref mut p) => p.position = pos,
            _ => ()
        }
    }
}
