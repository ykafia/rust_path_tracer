mod components;
use components::*;

use nalgebra as na;
use na::{Rotation3,Vector3};
use image::{
    Rgba,
    DynamicImage,
    GenericImage,
    Pixel,
    Pixels
};


fn main() {
    println!("Hello, world!");
    let result = render_scene(&Scene::new());
    result.save("render.png").unwrap();
}


fn render_scene(scene : &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    image = scene.fire_rays(&mut image);
    image
}
#[test]
fn name() {
    let result = render_scene(&Scene::new());
}