mod components;
use components::*;

use image::gif::Encoder;
use image::*;
use na::{Vector3};
use nalgebra as na;
use std::fs::File;
use std::thread;

fn main() {
    render_scene_before();
}
fn render_scene() {
    let elements = [
        Element::Sphere(Sphere::new(0f32, 0f32, -3f32, Colors::BLUE, 1.0)),
        Element::Sphere(Sphere::new(0f32, 1f32, -4f32, Colors::RED, 1.0)),
        Element::Sphere(Sphere::new(1f32, 1f32, -1f32, Colors::GREEN, 1.0)),
        Element::Plane(Plane::new()),
        Element::Triangle(Triangle::new_defined()),
        Element::Triangle(Triangle::new(
            [Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 5.0, 0.0),
            Vector3::new(5.0, 0.0, 5.0)],
            Colors::BLUE.value(),
            0.8
        ))
    ];
    let lights = [
        Light::DirectionalLight(DirectionalLight {
            color: Colors::SKYBLUE.value(),
            intensity: 0.4,
            direction: Vector3::new(0.0, -1.0, -1.0),
        }),
    ];
    let mut image = 
        DynamicImage::new_rgb8(600, 400);

    let scene = Scene::new(
        Vector3::new(-3.0,6.0,-5.0), 
        elements[0].get_position()
    );
    image = scene.rayon_rays(&mut image, &elements, &lights);
    image.save("./renders/rayon_image.jpg").expect("file saved")
}


fn render_scene_before() {
    let elements = [
        Element::Sphere(Sphere::new(0.,0., 0., Colors::BLUE, 0.8)),
        Element::Plane(Plane::new()),
        Element::Sphere(Sphere::new(1., 0., 1., Colors::WHITE, 0.7))
    ];
    let lights = [
        Light::DirectionalLight(DirectionalLight::new(
            Vector3::new(1.,-1.,1.),
            Colors::WHITE.value(),
            1.0
        ))
    ];
    let mut image = 
        DynamicImage::new_rgb8(600, 400);

    let scene = Scene::new(Vector3::new(3.0,6.0,3.0), Vector3::new(0.,0.,0.));
    image = scene.fire_rays(&mut image, &elements, &lights);
    image.save("./renders/rayon_image.png").expect("file saved")
}