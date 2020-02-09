mod components;
use components::*;

use image::{DynamicImage, GenericImage, Pixel, Pixels, Rgba};
use na::{Rotation3, Vector3};
use nalgebra as na;
use std::thread;


fn main() {
    println!("Rendering multiple scenes");   
    render_multiple_scenes(); 
}

fn render_multiple_scenes() {
    let mut threads = vec![];
    threads.push(thread::spawn(move || {
        let result = render_scene(
            &Scene::new(
                Vector3::new(8f32,2f32,-5f32)
            )
        );
        println!("Rendering 1 done");
        result.save("render1.png").unwrap();
        
    }));
    threads.push(thread::spawn(move || {
        let result = render_scene(
            &Scene::new(
                Vector3::new(0f32,0f32,0f32)
            )
        );
        println!("Rendering 2 done");
        result.save("render2.png").unwrap();
    }));
    threads.push(thread::spawn(move || {
        let result = render_scene(
            &Scene::new(
                Vector3::new(0f32,8f32,0f32)
            )
        );
        println!("Rendering 3 done");
        result.save("render3.png").unwrap();
    }));
    use std::time::Instant;
    let now = Instant::now();
    for child in  threads {
        
        let _ = child.join();
    }
    let elapsed = now.elapsed();
    println!("time elapsed {:?}", elapsed );
    
}
fn render_scene(scene: &Scene) -> DynamicImage {
    
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            image.put_pixel(x, y, Color::new(0u8, 0u8, 255u8, 255).to_rgba());
        }
    }
    image = scene.fire_rays(&mut image);
    image
}
#[test]
fn name() {
    render_scene(
        &Scene::new(
            Vector3::new(0f32,0f32,0f32)
        )
    );
}
