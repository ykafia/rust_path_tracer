use super::*;
use ecs::*;

mod ecs;


use legion::prelude::*;

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct TransformComponent{
    position : Vector3<f32>,
    rotation : Vector3<f32>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityComponent {
    velocity : Vector3<f32>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Model(usize);

#[derive(Clone, Copy, Debug)]
pub struct LightComponent(Light);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Static;

pub fn create_universe() -> Universe {
    let mut universe = Universe::new();
    universe
} 