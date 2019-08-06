extern crate cgmath;

use cgmath::{vec3, Vector3, InnerSpace};
use std::collections::HashMap;

mod display;
mod bvh;
mod geometry;
mod bbox;
mod ray;

use typed_arena::Arena;
use std::cell::RefCell;
use bvh::loader_rodent::*;

fn main() {
    let bvh = load_bvh_rodent("bvh.bin");
    display::open_window();
}