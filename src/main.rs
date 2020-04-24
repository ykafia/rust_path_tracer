
mod material;
mod components;
mod utils;
use components::*;
use material::*;
use utils::*;

use image::gif::Encoder;
use image::*;
use na::{Vector3};
use nalgebra as na;
use std::time::*;


fn main() {
    render_scene();
}
fn render_scene() {
    let elements = [
        Element::Sphere(Sphere::textured(0f32, 2f32, 0f32,"textures/rust_logo.png", 3.0,  0.5)),
        // Element::Sphere(Sphere::new(0f32, 1f32, -4f32,5.0, 0.7)),
        // Element::Sphere(Sphere::new(1f32, 1f32, -1f32,8.0,  0.6)),
        Element::Plane(Plane::textured()),
        // Element::Triangle(Triangle::new_defined()),
        Element::Triangle(Triangle::new(
            [
                Vector3::new(-6.0, 1.0, -8.0),
                Vector3::new(-6.0, 8.0, -8.0),
                Vector3::new(6.0, 1.0, -8.0),
                
            ],
            Colors::BLUE.value(),
            0.2
        ))
    ];
    let lights = [
        // Light::PointLight(PointLight {
        //     color: Colors::WHITE.value(),
        //     intensity: 4.0,
        //     position : Vector3::new(0.0, 2.0, 1.0),
        // }),
        // Light::PointLight(PointLight {
        //     color: Colors::RED.value(),
        //     intensity: 3.0,
        //     position : Vector3::new(0.0, 2.0, -1.0),
        // }),
        Light::DirectionalLight(DirectionalLight {
            color: Colors::WHITE.value(),
            intensity: 0.4,
            direction: Vector3::new(0.0, -1.0, -1.0),
        }),
    ];
    let mut image = 
        DynamicImage::new_rgb8(600, 400);

    let camera_pos = Vector3::new(10.0,5.0,10.0);
    let scene = Scene::new(
        camera_pos, 
        elements[0].get_position() - camera_pos
    );
    let first = Instant::now();
    image = scene.rayon_rays(&mut image, &elements, &lights);
    let last = Instant::now();
    println!("{} fps", 1.0/(last-first).as_secs_f64());
    image.save("./renders/rayon_image.png").expect("file saved")
}

