extern crate cgmath;

use cgmath::{vec3, Vector3, InnerSpace, vec2};
use std::collections::HashMap;

mod display;
mod bvh;
mod geometry;
mod bbox;
mod ray;
mod camera;
mod controller;
//mod test;

use typed_arena::Arena;
use std::cell::RefCell;
use bvh::loader_rodent::*;
use std::rc::Rc;
use std::borrow::Borrow;
use std::pin::Pin;
use crate::display::{Color, Display};
use crate::camera::Camera;
use crate::controller::Controller;
use minifb::Window;
use std::time::SystemTime;

fn main() {
    let result = load_bvh_rodent("bvh.bin");
    let bvh = &result.bvh;

    let mut controller = Controller::new(vec3(0.0, 0.0, 0.0), vec2(0.0, 0.0));
    let mut display = Display::new(320, 240);

    let mut then = SystemTime::now();
    while display.window().is_open() {
        controller.update(display.window());
        let camera = controller.to_camera(display.window());

        let shader = | window: &Window, x, y | {
            let s = x as f32 / window.get_size().0 as f32;
            let t = (window.get_size().1 as i32 - y) as f32 / window.get_size().1 as f32;
            let mut ray = camera.make_ray(s, t);
            let hit = bvh.trace(&mut ray, false);
            Color(f32::ln(ray.steps as f32) * 0.125, ray.steps as f32 / 64.0, if hit { 1.0 } else { 0.0 })
            //let z = f32::ln(ray.t_max) * 0.25; Color(z, z, z)
        };

        display.refresh(shader);

        let now = SystemTime::now();
        let delta = now.duration_since(then).unwrap();
        let fps = 1000_000.0 / (delta.as_micros() as f64);
        display.set_title(format!("fps: {}", fps).as_str());
        then = now;
    }
}