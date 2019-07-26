extern crate cgmath;

use cgmath::{vec3, Vector3, InnerSpace};
use std::collections::HashMap;

mod display;
mod bvh;
mod geometry;
mod bbox;

use bvh::loader_rodent::*;

#[derive(Debug)]
struct Cake {
    funny: i32,
    that: i64,
}

fn main() {
    let bvh = load_bvh_rodent("bvh.bin");
    display::open_window();
}
