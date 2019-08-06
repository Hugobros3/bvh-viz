extern crate cgmath;

use cgmath::{vec3, Vector3, InnerSpace};
use std::collections::HashMap;

mod display;
mod bvh;
mod geometry;
mod bbox;

use typed_arena::Arena;
use std::cell::RefCell;
use bvh::loader_rodent::*;

//#[derive(Debug)]
/*
impl<'a> Cake<'a> {
    fn add<'b>(&'b mut self, thing: Thing) -> &'a Thing {
        self.arena.alloc(thing)
    }
}*/

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
//fn simpleprocess<'b: 'a, 'a>(cake: &'b mut Cake<'a>) {
/*fn simpleprocess<'a: 'b, 'b>(cake: &'b mut Cake<'a>) {
    cake.that += cake.funny as i64;
}

#[derive(Debug)]
struct Thing {
    text: String,
}

struct Cake<'a> {
    funny: i32,
    that: i64,
    list: RefCell<Vec<&'a Thing>>,
    arena: Arena<Thing>,
}

fn subprocess<'a: 'b, 'b>(cake: &'b Cake<'a>) {
    let thing = Thing { text: format!("Fun fact {}", 7) };
    let r: &'a Thing = cake.arena.alloc(thing);
    cake.list.borrow_mut().push( r );
}

fn main() {
    let mut cake = Cake {
        funny: 32,
        that: 0_i64,
        list: RefCell::new(Vec::new()),
        arena: Arena::new(),
    };

    subprocess(&mut cake);
    println!("{:?}", cake.list.borrow().get(0))
}*/

fn main() {
    let bvh = load_bvh_rodent("bvh.bin");
    //display::open_window();
}



/*fn process<'a: 'b, 'b>(cake: &'a mut Cake<'b>) {
    cake.funny+=1;
    for i in 0..=3 {
        subprocess(cake);
    }
    cake.funny-=1;
}*/