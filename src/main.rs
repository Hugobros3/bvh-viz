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
use std::rc::Rc;
use std::borrow::Borrow;
use std::pin::Pin;

struct SelfRef<'a> {
    data: String,
    ptr: RefCell<Vec<&'a String>>,
}

fn mk_weird<'a>() -> Pin<Box<SelfRef<'a>>> {
    let s = Box::pin(SelfRef {
        data: format!("weird flex but ok"),
        ptr: RefCell::new(Vec::new())
    });
    {
        let borrowed: &SelfRef = s.borrow();
        borrowed.ptr.borrow_mut().push(&s.data);
    }
    return s;
}

fn weird() {

}

fn main() {
    let bvh = load_bvh_rodent("bvh.bin");
    display::open_window();
}