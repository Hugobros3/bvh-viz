extern crate minifb;

use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub struct Color(pub f32, pub f32, pub f32);
pub type Shader = fn(&Window, i32, i32) -> Color;

pub fn open_window(shader: Shader) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i,d) in buffer.iter_mut().enumerate() {
            let x = i % window.get_size().0;
            let y = i / window.get_size().0;
            let color = shader(&window, x as i32, y as i32);
            let r = clamp((color.0 * 256.0) as u32, 0, 255);
            let g = clamp((color.1 * 256.0) as u32, 0, 255);
            let b = clamp((color.2 * 256.0) as u32, 0, 255);
            *d = r << 16 | g << 8 | b << 0; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}

fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min { min } else if v > max { max } else { v }
}