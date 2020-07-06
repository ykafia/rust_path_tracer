
mod material;
mod graphics;
mod utils;
mod window;
mod render;
mod components;

use graphics::*;
use material::*;
use utils::*;
use window::*;
use render::*;
use components::*;
use rand::prelude;
use rand::Rng;

use image::gif::Encoder;
use image::*;
use na::{Vector3};
use nalgebra as na;
use std::time::*;


fn main() {
    run(create_scene())
}

pub fn create_scene() -> Scene {
    let mut rng = rand::thread_rng();
    let mut scene = Scene::new(Vector3::new(5f32,2f32,5f32), Vector3::new(0f32,0f32,0f32), 800,640);
    scene.world.insert(
        (),
        (0..4).map(|_| (
            TransformComponent {
                position : Vector3::new(
                                rng.gen_range(-1.0f32,1.0f32),
                                rng.gen_range(-1f32,1.0f32),
                                rng.gen_range(-1.0f32,1.0f32)
                        ),
                rotation : Vector3::new(1f32,0f32,0f32)
            },
            RenderableComponent(Element::Sphere(Sphere {
                // `center`, `material`, `radius`
                center : Vector3::zeros(),
                radius : rng.gen_range(1f32,5.0f32),
                material : Material {
                    // `albedo`, `emissive`, `reflectivity`
                    albedo : 1.0,
                    reflectivity : None,
                    emissive : Surface::Color(Colors::BLACK.value())
                },
            }))
        
        ))
    );
    scene.world.insert(
        (),
        (0..4).map(|_| (
            TransformComponent {
                position : Vector3::new(
                                rng.gen_range(-1.0f32,1.0f32),
                                rng.gen_range(-1f32,1.0f32),
                                rng.gen_range(-1.0f32,1.0f32)
                        ),
                rotation : Vector3::new(1f32,0f32,0f32)
            },
            RenderableComponent(Element::Plane(Plane {
                // `center`, `material`, `radius`
                normal : Vector3::y(),
                position : Vector3::zeros(),
                material : Material {
                    // `albedo`, `emissive`, `reflectivity`
                    albedo : 1.0,
                    reflectivity : None,
                    emissive : Surface::Color(Colors::BLACK.value())
                },
                repeat_texture : None
            }))
        
        ))
    );
    scene
}