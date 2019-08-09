extern crate cgmath;

use cgmath::{vec3, Vector3, InnerSpace};
use std::collections::HashMap;

mod display;
mod bvh;
mod geometry;
mod bbox;
mod ray;
mod test;

use typed_arena::Arena;
use std::cell::RefCell;
use bvh::loader_rodent::*;
use std::rc::Rc;
use std::borrow::Borrow;
use std::pin::Pin;
use crate::display::{Shader, Color};

fn main() {
    //let oof = &weird().damn.data;
    //let bvh = load_bvh_rodent("bvh.bin");
    let shader: Shader = | window, x, y | {
        Color(x as f32 / 256.0, y as f32 / 256.0, 0.0)
    };
    display::open_window(shader);
}