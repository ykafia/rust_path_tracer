use super::na::{Vector3,Rotation3};
use super::*;




pub struct Camera {
    pub position : Vector3<f32>,
    pub rotation : Rotation3<f32>,
    pub width : u32,
    pub height : u32,
    pub fov : f32,
    pub sphere : Sphere
}

