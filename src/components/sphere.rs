use super::na::{Vector3,Rotation3};
use super::utils::Color;

pub struct Sphere {
    pub center : Vector3<f32>,
    pub color : Color,
    pub radius : f32
}