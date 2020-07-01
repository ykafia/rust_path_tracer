use image::*;
use minifb::*;
use super::*;
use rayon::iter::{IntoParallelRefIterator,IntoParallelIterator,ParallelIterator, IndexedParallelIterator};
use rayon::iter::Chunks;


const W: usize = 640;
const H: usize = 480;

pub fn window() {

    
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
    println!("First size {}", u32_buffer.len());
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        u32_buffer =
            render_scene(H,W)
                .into_par_iter()
                .collect();
        println!("{}",u32_buffer.len());
        window
            .update_with_buffer(&u32_buffer, W, H)
            .unwrap();
    }

}