extern crate minifb;

use minifb::{Key, WindowOptions, Window};
use std::time::SystemTime;
use rayon::prelude::*;

pub struct Display {
    window: Window,
    buffer: Vec<u32>,
    pub width: i32,
    pub height: i32,
}

pub struct Color(pub f32, pub f32, pub f32);

impl Display {
    pub fn new(width: i32, height: i32) -> Self {
        let buffer: Vec<u32> = vec![0; (width * height) as usize];
        let options = WindowOptions {
            resize: false,
            ..WindowOptions::default()
        };
        let window = Window::new("",width as usize,height as usize, options).unwrap_or_else(|e| { panic!("{}", e); });

        Display {
            window,
            buffer,
            width,
            height,
        }
    }

    pub fn refresh<F> (&mut self, shader: F)
    where F: Fn((usize, usize), i32, i32) -> Color + Sync {
        let size = self.window.get_size();
        self.buffer.par_iter_mut().enumerate().for_each(|(i, d)| {
            let x = i % size.0;
            let y = i / size.0;
            let color = shader(size, x as i32, y as i32);
            *d = rgb(&color);
        });

        /*for (i,d) in self.buffer.par_iter_mut().enumerate() {
            let x = i % self.window.get_size().0;
            let y = i / self.window.get_size().0;
            let color = shader(&self.window, x as i32, y as i32);
            *d = rgb(&color);
        }*/
        self.window.update_with_buffer(&self.buffer).unwrap();
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn window(&self) -> &Window {
        return &self.window;
    }
}

fn rgb(color: &Color) -> u32 {
    let r = clamp((color.0 * 256.0) as u32, 0, 255);
    let g = clamp((color.1 * 256.0) as u32, 0, 255);
    let b = clamp((color.2 * 256.0) as u32, 0, 255);
    r << 16 | g << 8 | b << 0
}

fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min { min } else if v > max { max } else { v }
}