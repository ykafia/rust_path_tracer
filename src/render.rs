use super::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub fn render_scene(height : usize, width : usize) -> Vec<u32>{
    let elements = [
        Element::Sphere(Sphere::new(-6f32, 5f32, 0f32,Colors::CYAN, 3.0,  1.0)),
        Element::Sphere(Sphere::new(0f32, 8f32, -4f32,Colors::MAGENTA,5.0, 0.7)),
        // Element::Sphere(Sphere::new(12f32, 1f32, -12f32,Colors::GREEN,8.0,  0.6)),
        Element::Plane(Plane::new()),
        // Element::Triangle(Triangle::new_defined()),
        // Element::Triangle(Triangle::new(
        //     [
        //         Vector3::new(-6.0, 1.0, -8.0),
        //         Vector3::new(-6.0, 8.0, -8.0),
        //         Vector3::new(6.0, 1.0, -8.0),
                
        //     ],
        //     Colors::BLUE.value(),
        //     0.2
        // ))
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
            intensity: 1.0,
            direction: Vector3::new(0.0, -1.0, -1.0),
        }),
    ];
    let mut image = 
        DynamicImage::new_rgb8(width as u32, height as u32);

    let camera_pos = Vector3::new(10.0,5.0,10.0);
    let scene = Scene::new(
        camera_pos, 
        elements[0].get_position() - camera_pos
    );
    let first = Instant::now();
    image = scene.rayon_rays(&mut image, &elements, &lights);
    let last = Instant::now();
    println!("{} fps", 1.0/(last-first).as_secs_f64());
    // image.save("./renders/rayon_image.png").expect("file saved")
    image
        .to_rgb()
        .pixels()
        .map(|p| p.to_rgba().0)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect()
}

