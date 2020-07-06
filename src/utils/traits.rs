use super::*;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<PointInfo>;
    fn simple_intersect(&self, ray:&Ray) -> bool;
    fn get_color(&self, intersection : Vector3<f32>) -> Color;
    fn get_position(&self) -> Vector3<f32>;
    fn get_albedo(&self) -> f32;
    fn get_texcoord(&self,intersect : Vector3<f32>) -> TexCoord;
    fn get_reflectivity(&self) -> Option<f32>;
}

pub trait ECSIntersectable {
    fn intersect_ecs(&self, ray: &Ray, transform : &TransformComponent) -> Option<PointInfo>;
    fn simple_intersect_ecs(&self, ray:&Ray, transform : &TransformComponent) -> bool;
    fn get_color_ecs(&self, intersection : Vector3<f32>, transform : &TransformComponent) -> Color;
    fn get_albedo_ecs(&self) -> f32;
    fn get_texcoord_ecs(&self,intersect : Vector3<f32>, transform : &TransformComponent) -> TexCoord;
    fn get_reflectivity_ecs(&self) -> Option<f32>;
}