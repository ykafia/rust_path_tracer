use image::*;
use minifb::*;
use super::*;
use rayon::iter::{IntoParallelRefIterator,IntoParallelIterator,ParallelIterator, IndexedParallelIterator};
use rayon::iter::Chunks;


const W: usize = 640;
const H: usize = 480;

pub fn window(scene : &mut Scene) {

    
    // Allocate the output buffer.
    let mut buf = vec![115u8; 3*W*H];
    // Read the next frame. Currently this function should only called once.
    // The default options
    // convert buffer to u32

    let mut u32_buffer: Vec<u32> = 
        buf
        .into_par_iter()
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();
    
    let mut window = Window::new(
        "Noise Test - Press ESC to exit",
        W,
        H,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");
    let mut elapsed = 0f64;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let first = Instant::now(); 
        if window.is_key_down(Key::Left){
            scene.camera.position.z += 5f32*elapsed as f32;
        }
        if window.is_key_down(Key::Right){
            scene.camera.position.z -= 5f32*elapsed as f32;
        }
        if window.is_key_down(Key::Up){
            scene.camera.position.x += 5f32*elapsed as f32;
        }
        if window.is_key_down(Key::Down){
            scene.camera.position.x -= 5f32*elapsed as f32;
        }
        if window.is_key_down(Key::Space){
            scene.camera.position.y += 5f32*elapsed as f32;
        }
        if window.is_key_down(Key::C){
            scene.camera.position.y -= 5f32*elapsed as f32;
        }

        scene.camera.change_rotation(-scene.camera.position);
        
        u32_buffer = 
            scene
            .rayon_rays()
            .to_rgb()
            .pixels()
            .map(|p| p.to_rgba().0)
            .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
            .collect();      
        window
            .update_with_buffer(&u32_buffer, scene.camera.width, scene.camera.height)
            .unwrap();
        let last = Instant::now();
        elapsed = (last-first).as_secs_f64();
        println!("{:.1} fps", 1.0/elapsed);
    }

}