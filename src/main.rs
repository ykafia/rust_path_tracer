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
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::new(x, y, scene);

            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, scene.sphere.color.to_rgba())
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}
#[test]
fn name() {
    let result = render_scene(&Scene::new());
}