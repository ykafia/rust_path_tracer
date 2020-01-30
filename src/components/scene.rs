use super::na::{Vector3};
use super::*;


pub struct Scene {
    pub width : u32,
    pub height : u32,
    pub fov : f32,
    pub sphere : Sphere
    
    //pub spheres :  Vec<Sphere>
}

impl Scene {
    pub fn new() -> Scene {
        Scene{
            width : 600,
            height : 400,
            fov : 90.0,
            sphere : Sphere {
                center: Vector3::new(
                    0.0,
                    0.0,
                    -5.0,
                ),
                radius: 1.0,
                color: Color {
                    r: 125,
                    g: 125,
                    b: 225,
                    a: 0
                },
            }
        }
    }
}