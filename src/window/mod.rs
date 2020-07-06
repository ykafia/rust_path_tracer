use image::*;
use minifb::*;
use super::*;
use rayon::iter::{IntoParallelRefIterator,IntoParallelIterator,ParallelIterator, IndexedParallelIterator};
use rayon::iter::Chunks;




pub fn run(scene : Scene) {
    let (W,H) = (scene.camera.width,scene.camera.height);
    
    // Allocate the output buffer.
    let mut buf = vec![115u8; 3*W*H];
    let mut image = DynamicImage::new_rgba8(W as u32, H as u32);
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
        "Thracer - Press ESC to exit",
        W,
        H,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let first = Instant::now(); 
        u32_buffer = scene
                        .ecs_rays(&mut image)
                        .to_rgb()
                        .pixels()
                        .map(|p| p.to_rgba().0)
                        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
                        .collect();      
        window
            .update_with_buffer(&u32_buffer, W, H)
            .unwrap();
        let last = Instant::now();
        println!("{:.1} fps", 1.0/(last-first).as_secs_f64());
    }

}