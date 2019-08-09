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
mod test;

use typed_arena::Arena;
use std::cell::RefCell;
use bvh::loader_rodent::*;
use std::rc::Rc;
use std::borrow::Borrow;
use std::pin::Pin;
use crate::display::{Shader, Color};
use crate::camera::Camera;
use crate::controller::Controller;
use minifb::Window;

fn main() {
    //let oof = &weird().damn.data;
    //let result = load_bvh_rodent("bvh.bin");
    let result = load_bvh_rodent("C:\\Users\\Gobrosse\\AppData\\Local\\Packages\\CanonicalGroupLimited.Ubuntu18.04onWindows_79rhkp1fndgsc\\LocalState\\rootfs\\home\\hugo\\git\\anydsl2\\rodent\\build3\\data\\bvh.bin");
    let bvh = &result.bvh;

    let controller = Controller::new(vec3(0.0,0.0,0.0), vec2(0.0, 0.0));
    let shader = | window: &Window, x, y | {
        let camera = controller.to_camera(window);
        let s = x as f32 / window.get_size().0 as f32;
        let t = (window.get_size().1 as i32 - y) as f32 / window.get_size().1 as f32;
        let mut ray = camera.make_ray(s, t);
        let hit = bvh.trace(&mut ray, false);
        //Color(f32::ln(ray.steps as f32) * 0.125, ray.steps as f32 / 32.0, if hit { 1.0 } else { 0.0 })
        let z = f32::ln(ray.t_max) * 0.25;
        Color(z, z, z)
    };
    display::open_window(shader);
}