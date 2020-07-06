use super::*;
use ecs::*;

mod ecs;


use legion::prelude::*;

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct TransformComponent{
    pub position : Vector3<f32>,
    pub rotation : Vector3<f32>
}

#[derive(Clone,Copy,Debug)]
pub struct IDComponent(pub usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityComponent {
    pub velocity : Vector3<f32>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Model(usize);

#[derive(Clone, Copy, Debug)]
pub struct LightComponent(pub Light);


pub struct RenderableComponent(pub Element);


#[derive(Clone, Copy, Debug, PartialEq)]
struct Static; 


impl Intersectable for RenderableComponent {

    fn get_position(&self) -> Vector3<f32>{
        self.0.get_position()
    }
    
    fn simple_intersect(&self, ray: &Ray) -> bool {
        self.0.simple_intersect(ray)
    }

    fn intersect(&self, ray : &Ray) -> Option<PointInfo> {
        self.0.intersect(ray)
    }
    fn get_color(&self, intersection: Vector3<f32>) -> Color {
        self.0.get_color(intersection)
    }

    fn get_albedo(&self) -> f32 {
        self.0.get_albedo()
    }
    fn get_reflectivity(&self) -> Option<f32> {
        self.get_reflectivity()
    }

    fn get_texcoord(&self, intersect: Vector3<f32>) -> TexCoord {
        self.0.get_texcoord(intersect)
    }
}
impl ECSIntersectable for RenderableComponent {

    fn simple_intersect_ecs(&self, ray: &Ray, transform : &TransformComponent) -> bool {
        self.0.simple_intersect_ecs(ray, transform)
    }

    fn intersect_ecs(&self, ray : &Ray, transform : &TransformComponent) -> Option<PointInfo> {
        self.0.intersect_ecs(ray, transform)
    }
    fn get_color_ecs(&self, intersection: Vector3<f32>, transform : &TransformComponent) -> Color {
        self.0.get_color_ecs(intersection, transform)
    }

    fn get_albedo_ecs(&self) -> f32 {
        self.0.get_albedo_ecs()
    }
    fn get_reflectivity_ecs(&self) -> Option<f32> {
        self.get_reflectivity_ecs()
    }

    fn get_texcoord_ecs(&self, intersect: Vector3<f32>, transform : &TransformComponent) -> TexCoord {
        self.0.get_texcoord_ecs(intersect, transform)
    }
}

impl LightComponent {
    pub fn get_direction(&self, transform : &TransformComponent, element: &TransformComponent) -> Vector3<f32> {
        match self.0 {
            Light::DirectionalLight(_) => transform.rotation,
            Light::PointLight(_) => element.position - transform.position,
        }
    }
    pub fn get_color(&self) -> Color {
        self.0.get_color()
    }
    pub fn get_intensity(&self, intersection : Vector3<f32>) -> f32 {
        self.0.get_intensity(intersection)
    }
}