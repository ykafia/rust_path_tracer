use super::*;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<PointInfo>;
    fn simple_intersect(&self, ray:&Ray) -> bool;
    fn get_color(&self, intersection : Vector3<f32>) -> Color;
    fn get_position(&self) -> Vector3<f32>;
    fn get_albedo(&self) -> f32;
    fn get_texcoord(&self,intersect : Vector3<f32>) -> TexCoord;
}