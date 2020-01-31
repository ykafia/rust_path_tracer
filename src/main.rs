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
    for x in 0..scene.width{
        for y in 0..scene.height {
            image.put_pixel(x, y, Color::new(0u8,0u8,255u8,255).to_rgba());
        }
    }
    image = scene.fire_rays(&mut image);
    image
}
#[test]
fn name() {
    let result = render_scene(&Scene::new());
}