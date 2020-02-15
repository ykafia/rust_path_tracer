mod components;
use components::*;

use image::*;
use image::gif::{Decoder, Encoder};
use na::{Rotation3, Vector3};
use nalgebra as na;
use std::thread;
use std::fs::File;

fn main() {
    
    // dotprod();
    //render_multiple_scenes();
    render_animation();
}

fn render_multiple_scenes() {
    let mut threads = vec![];
    let elements = [
        Element::Sphere(Sphere::new(0f32, 0f32, -3f32,Colors::BLUE,1.0)),
        Element::Sphere(Sphere::new(0f32, 1f32, -4f32,Colors::RED,1.0)),
        Element::Sphere(Sphere::new(1f32, 1f32, -1f32,Colors::GREEN,1.0)),
        Element::Plane(Plane::new())
    ];
    threads.push(thread::spawn(move || {
        let camerapos = Vector3::new(8f32, 2f32, -5f32);
        let result = render_scene(
            &Scene::new(
                camerapos,
                &elements[0].get_position() - camerapos
            ),
            &elements
        );
        println!("Rendering 1 done");
        result.save("render1.png").unwrap();
    }));
    threads.push(thread::spawn(move || {
        let camerapos = Vector3::new(2f32, 1f32, -6f32);
        let result = render_scene(
            &Scene::new(
                camerapos,
                &elements[0].get_position() - camerapos
            ),
            &elements
        );
        println!("Rendering 2 done");
        result.save("render2.png").unwrap();
    }));
    threads.push(thread::spawn(move || {
        let camerapos = Vector3::new(0f32, 8f32, 0f32);
        let result = render_scene(
            &Scene::new(
                camerapos,
                &elements[0].get_position() - camerapos
            ),
            &elements
        );
        println!("Rendering 3 done");
        result.save("render3.png").unwrap();
    }));
    use std::time::Instant;
    let now = Instant::now();
    for child in threads {
        let _ = child.join();
    }
    let elapsed = now.elapsed();
    println!("time elapsed {:?}", elapsed);
}

fn render_animation() {
    println!("Rendering animation");
    let init_pos = Vector3::new(0.0, 0.0, 0.0);
    let b = Vector3::new(1.0, 1.0, 0.0);
    let c = Vector3::new(2.0, 1.0, 1.0);
    let d = Vector3::new(2.5, 1.0, 3.0);
    let mut frames: Vec<Frame> = Vec::new();
    let elements = [
        Element::Sphere(Sphere::new(0f32, 0f32, -3f32,Colors::BLUE,1.0)),
        Element::Sphere(Sphere::new(0f32, 1f32, -4f32,Colors::RED,1.0)),
        Element::Sphere(Sphere::new(1f32, 1f32, -1f32,Colors::GREEN,1.0)),
        Element::Plane(Plane::new())
    ];
    let mut scene = Scene::new(init_pos, &elements[0].get_position()-init_pos);
    for i in (0..100).step_by(4) {
        let mut threads = vec![];
        threads.push(thread::spawn(move || {
            scene.camera.position = catmull(init_pos, b, c, d, i as f32 / 1000.0);
            scene.camera.change_rotation(&elements[0].get_position() - scene.camera.position);
            Frame::new(
                render_scene(&scene,&elements)
                    .as_rgba8()
                    .expect("Converted dynamic image to rgba8")
                    .clone(),
            )
        }));
        threads.push(thread::spawn(move || {
            scene.camera.position = catmull(init_pos, b, c, d, i as f32 +1.0 / 1000.0);
            scene.camera.change_rotation(&elements[0].get_position() - scene.camera.position);
            Frame::new(
                render_scene(&scene,&elements)
                    .as_rgba8()
                    .expect("Converted dynamic image to rgba8")
                    .clone(),
            )
        }));
        threads.push(thread::spawn(move || {
            scene.camera.position = catmull(init_pos, b, c, d, i as f32 +2.0 / 1000.0);
            scene.camera.change_rotation(&elements[0].get_position() - scene.camera.position);
            Frame::new(
                render_scene(&scene, &elements)
                    .as_rgba8()
                    .expect("Converted dynamic image to rgba8")
                    .clone(),
            )
        }));
        threads.push(thread::spawn(move || {
            scene.camera.position = catmull(init_pos, b, c, d, i as f32 +3.0 / 1000.0);
            scene.camera.change_rotation(&elements[0].get_position() - scene.camera.position);
            Frame::new(
                render_scene(&scene, &elements)
                    .as_rgba8()
                    .expect("Converted dynamic image to rgba8")
                    .clone(),
            )
        }));
        for t in threads {
            frames.push(t.join().expect("Frame was done"));
        }
    }
    let mut encoder = Encoder::new(File::create("out.gif").unwrap());
    encoder.encode_frames(frames.into_iter()).unwrap();

}
fn render_scene(scene: &Scene, elements : &[Element]) -> DynamicImage {
    let mut image = DynamicImage::new_rgba8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            image.put_pixel(x, y, Colors::BLACK.value().to_rgba());
        }
    }
    
    image = scene.fire_rays(&mut image,elements);
    image
}
