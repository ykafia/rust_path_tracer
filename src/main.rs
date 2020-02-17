mod components;
use components::*;

use image::gif::Encoder;
use image::*;
use na::{Rotation3, Vector3};
use nalgebra as na;
use std::fs::File;
use std::thread;

fn main() {
    // dotprod();
    render_multiple_scenes();
    // render_animation();
}

fn render_multiple_scenes() {
    let mut threads = vec![];
    let elements = [
        Element::Sphere(Sphere::new(0f32, 0f32, -3f32, Colors::BLUE, 1.0)),
        Element::Sphere(Sphere::new(0f32, 1f32, -4f32, Colors::RED, 1.0)),
        Element::Sphere(Sphere::new(1f32, 1f32, -1f32, Colors::GREEN, 1.0)),
        Element::Plane(Plane::new()),
        // Element::Triangle(Triangle::new_defined())
    ];
    let lights = [
        Light::DirectionalLight(DirectionalLight {
            color: Colors::WHITE.value(),
            intensity: 0.001,
            direction: Vector3::new(0.0, -1.0, -1.0),
        }),
        Light::PointLight(PointLight {
            color: Colors::WHITE.value(),
            intensity: 2.0,
            position: Vector3::new(0.0, 3.0, -6.5),
        }),
        Light::PointLight(PointLight {
            color: Colors::RED.value(),
            intensity: 1.0,
            position: Vector3::new(1.0, 3.0, -2.5),
        }),
    ];
    threads.push(thread::spawn(move || {
        let camerapos = Vector3::new(8f32, 2f32, -5f32);
        let result = render_scene(
            &Scene::new(camerapos, &elements[0].get_position() - camerapos),
            &elements,
            &lights,
        );
        println!("Rendering 1 done");
        result.save("render1.png").unwrap();
    }));
    threads.push(thread::spawn(move || {
        let camerapos = Vector3::new(2f32, 1f32, -6f32);
        let result = render_scene(
            &Scene::new(camerapos, &elements[0].get_position() - camerapos),
            &elements,
            &lights,
        );
        println!("Rendering 2 done");
        result.save("render2.png").unwrap();
    }));
    threads.push(thread::spawn(move || {
        let camerapos = Vector3::new(0f32, 8f32, 0f32);
        let result = render_scene(
            &Scene::new(camerapos, &elements[0].get_position() - camerapos),
            &elements,
            &lights,
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
    let init_pos = Vector3::new(-3.0, 1.0, 0.0);
    let b = Vector3::new(-2.0, 1.0, 0.0);
    let c = Vector3::new(8.0, 1.0, 3.0);
    let d = Vector3::new(9.0, 1.0, 6.0);
    let mut frames: Vec<Frame> = Vec::new();
    let elements = [
        Element::Sphere(Sphere::new(0f32, 0f32, -3f32, Colors::BLUE, 1.0)),
        Element::Sphere(Sphere::new(0f32, 1f32, -4f32, Colors::RED, 1.0)),
        Element::Sphere(Sphere::new(1f32, 1f32, -1f32, Colors::GREEN, 1.0)),
        Element::Plane(Plane::new()),
        Element::Triangle(Triangle::new_defined())
    ];
    let mut lights = [
        Light::DirectionalLight(DirectionalLight {
            direction: Vector3::new(0.0, -1.0, -1.0),
            color: Colors::WHITE.value(),
            intensity: 1.0,
        }),
        Light::PointLight(PointLight {
            position: Vector3::new(0.0, 1.0, 1.0),
            color: Colors::WHITE.value(),
            intensity: 1.0,
        }),
    ];
    let mut scene = Scene::new(
        Vector3::new(0.5, 1.5, 2.0),
        &elements[0].get_position() - init_pos,
    );
    for i in (0..300).step_by(4) {
        let mut threads = vec![];
        threads.push(thread::spawn(move || {
            lights[1].set_position(catmull(init_pos, b, c, d, i as f32 / 300.0));
            scene
                .camera
                .change_rotation(&elements[0].get_position() - scene.camera.position);

            (
                Quarter::TOPLEFT,
                render_quarter_scene(&scene, &elements, &lights, Quarter::TOPLEFT)
                    .clone(),
            )
        }));
        threads.push(thread::spawn(move || {
            lights[1].set_position(catmull(init_pos, b, c, d, i as f32 / 300.0));
            scene
                .camera
                .change_rotation(&elements[0].get_position() - scene.camera.position);

            (
                Quarter::TOPRIGHT,
                render_quarter_scene(&scene, &elements, &lights, Quarter::TOPRIGHT)
                    .clone(),
            )
        }));
        threads.push(thread::spawn(move || {
            lights[1].set_position(catmull(init_pos, b, c, d, i as f32 / 300.0));
            scene
                .camera
                .change_rotation(&elements[0].get_position() - scene.camera.position);
            (
                Quarter::BOTTOMLEFT,
                render_quarter_scene(&scene, &elements, &lights, Quarter::BOTTOMLEFT)
                    .clone(),
            )
        }));
        threads.push(thread::spawn(move || {
            lights[1].set_position(catmull(init_pos, b, c, d, i as f32 / 300.0));
            scene
                .camera
                .change_rotation(&elements[0].get_position() - scene.camera.position);

            (
                Quarter::BOTTOMRIGHT,
                render_quarter_scene(&scene, &elements, &lights, Quarter::BOTTOMRIGHT)
                    .clone(),
            )
        }));
        let mut images =  Vec::new();
        for t in threads {
            images.push(t.join().expect("Adding a resulted image in the list"));
            //frames.push(t.join().expect("Frame was done"));
        }
        frames.push(Frame::new(
            ImgParts::new(images)
                .get_result_image()
                .into_rgba(),
        ))
    }
    let mut encoder = Encoder::new(File::create("out.gif").unwrap());
    encoder.encode_frames(frames.into_iter()).unwrap();
}
fn render_scene(scene: &Scene, elements: &[Element], lights: &[Light]) -> DynamicImage {
    let mut image = DynamicImage::new_rgba8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            image.put_pixel(x, y, Colors::BLACK.value().to_rgba());
        }
    }

    image = scene.fire_rays(&mut image, elements, lights);
    image
}

fn render_quarter_scene(
    scene: &Scene,
    elements: &[Element],
    lights: &[Light],
    quarter: Quarter,
) -> DynamicImage {
    let mut image = DynamicImage::new_rgba8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            image.put_pixel(x, y, Colors::BLACK.value().to_rgba());
        }
    }

    image = scene.fire_quarter_rays(&mut image, elements, lights, quarter);
    image
}
