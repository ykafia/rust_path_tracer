
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

fn main() {
    render_scene();
}
fn render_scene() {
    let elements = [
        Element::Sphere(Sphere::new(0f32, 0f32, -3f32, 3.0, Colors::BLUE, 0.5)),
        Element::Sphere(Sphere::new(0f32, 1f32, -4f32,5.0, Colors::RED, 0.7)),
        Element::Sphere(Sphere::new(1f32, 1f32, -1f32,8.0, Colors::GREEN, 0.6)),
        Element::Plane(Plane::new()),
        // Element::Triangle(Triangle::new_defined()),
        // Element::Triangle(Triangle::new(
        //     [Vector3::new(0.0, 0.0, 0.0),
        //     Vector3::new(0.0, 1.0, 0.0),
        //     Vector3::new(1.0, 0.0, 1.0)],
        //     Colors::BLUE.value(),
        //     0.2
        // ))
    ];
    let lights = [
        Light::PointLight(PointLight {
            color: Colors::WHITE.value(),
            intensity: 4.0,
            position : Vector3::new(0.0, 2.0, 1.0),
        }),
        Light::PointLight(PointLight {
            color: Colors::RED.value(),
            intensity: 3.0,
            position : Vector3::new(0.0, 2.0, -1.0),
        }),
        Light::DirectionalLight(DirectionalLight {
            color: Colors::WHITE.value(),
            intensity: 0.3,
            direction: Vector3::new(0.0, -1.0, -1.0),
        }),
    ];
    let mut image = 
        DynamicImage::new_rgb8(600, 400);

    let camera_pos = Vector3::new(-0.0,3.0,-1.0);
    let scene = Scene::new(
        camera_pos, 
        elements[0].get_position() - camera_pos
    );
    image = scene.rayon_rays(&mut image, &elements, &lights);
    image.save("./renders/rayon_image.png").expect("file saved")
}

