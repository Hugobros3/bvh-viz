use super::either::Either;
use crate::bvh::{BvhTree, NodeId};
use crate::bvh::inner_arities::*;
use crate::bvh::leaf_arities::*;
use crate::geometry::*;
use std::fs::File;
use std::io::{BufReader, Read, Cursor};
use std::iter::Map;
use byteorder::{LittleEndian, ReadBytesExt};
use core::mem;
use std::io;
use compress::lz4;
use compress::lz4::Decoder;
use std::collections::HashMap;
use crate::bbox::{BBox, enclosing_bbox};
use cgmath::Vector3;
use typed_arena::Arena;
use std::convert::TryInto;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::{BorrowMut, Borrow};
use std::marker::PhantomData;
use std::pin::Pin;

/// describes the BVHs typically used by Rodent on the CPU side
type RodentBvh4_8<'a> = BvhTree<'a, Triangle, InnerNode8, LeafNode4<Triangle>>;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Node8 {
    bounds: [[f32; 8]; 6],
    child: [i32; 8],
    pad: [i32; 8],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Tri4 {
    v0: [[f32; 4]; 3],
    e1: [[f32; 4]; 3],
    e2: [[f32; 4]; 3],
    n: [[f32; 4]; 3],
    prim_id: [i32; 4],
    geom_id: [i32; 4],
}

pub fn load_bvh_rodent<'a>(filename: &str) -> Result {
    let f = File::open(filename).expect("File not found");
    let mut reader = BufReader::new(f);

    let inner_node_struct_size: i32 = read_i32(&mut reader);
    let leaf_node_struct_size: i32 = read_i32(&mut reader);
    println!("{}", inner_node_struct_size);
    println!("{}", leaf_node_struct_size);

    if (inner_node_struct_size == 256) {
        let nodes_buffer = read_buffer(&mut reader);
        let prims_buffer = read_buffer(&mut reader);

        //crudely shove raw vec<u8>s into vector of the c-structs
        let mut node8vec: Vec<Node8> = Vec::new();
        {
            let mut cursor = Cursor::new(&nodes_buffer);
            for i in 0..nodes_buffer.len() as i32 / inner_node_struct_size {
                let node8: Node8 = unsafe { std::ptr::read((nodes_buffer[((i * inner_node_struct_size) as usize)..].as_ptr() as *const _)) };
                node8vec.push(node8);
                //println!("Read {:?}", node8vec.last());
            }
        }
        let mut tri4vec: Vec<Tri4> = Vec::new();
        {
            let mut cursor = Cursor::new(&prims_buffer);
            for i in 0..prims_buffer.len() as i32 / leaf_node_struct_size {
                let tri4: Tri4 = unsafe { std::ptr::read((prims_buffer[((i * leaf_node_struct_size) as usize)..].as_ptr() as *const _)) };
                tri4vec.push(tri4);
                //println!("Read {:?}", tri4vec.last());
            }
        }

        //build the proper structs out of that
        let mut inner_nodes: Vec<InnerNode8> = Vec::new();
        let mut leaf_nodes: Vec<LeafNode4<Triangle>> = Vec::new();

        let mut map: HashMap<i32, NodeId> = HashMap::new();

        let node8_root = node8vec[0];
        //let conversion_env = Rc::new(Box::new(ConversionEnv {
        let conversion_env = Box::pin(ConversionEnv {
            node8vec,
            tri4vec,
            inner_nodes: RefCell::new(inner_nodes),
            leaf_nodes: RefCell::new(leaf_nodes),
            triangles: Arena::new(),
        });

        let root_node_id;
        {
            root_node_id = write_inner_node(node8_root, &conversion_env);
        }

        if let NodeId::Inner(iid) = root_node_id {
            println!("Read everything Ok ! {}", nodes_buffer.len());
            println!("{:?}", conversion_env.inner_nodes.borrow().get(iid as usize).unwrap());

            //println!("{:?}", &conversion_env.inner_nodes.borrow().iter());

            let result = unsafe {
                Result {
                    bvh: RodentBvh4_8 {
                        inner_nodes: &*((conversion_env.inner_nodes.as_ptr())),
                        leaf_nodes: &*((conversion_env.leaf_nodes.as_ptr())),
                        root_node_id,
                        terrible: PhantomData
                    },
                    data: conversion_env,
                }
            };
            return result;
        } else {
            panic!("Something went wrong");
        }
    } else {
        panic!("Unsupported inner node struct size: {}", inner_node_struct_size);
    }
}

pub struct Result<'a> {
    pub bvh: RodentBvh4_8<'a>,
    data: Pin<Box<ConversionEnv>>,
}

struct ConversionEnv {
    node8vec: Vec<Node8>,
    tri4vec: Vec<Tri4>,
    inner_nodes: RefCell<Vec<InnerNode8>>,
    leaf_nodes: RefCell<Vec<LeafNode4<Triangle>>>,
    triangles: Arena<Triangle>,
}

fn write_leaf_node<'b>(bbox: BBox, id: i32, tri4s: &Vec<Tri4>, leaf_nodes: &'b mut Vec<LeafNode4<Triangle>>, triangles: &'b Arena<Triangle>) -> NodeId {
    let mut count = 0;
    let mut prim_triangles: Vec<&Triangle> = Vec::new();

    let mut id = id as usize;
    'outer: loop {
        let tri4 = tri4s.get(id).unwrap();
        for i in 0..4 {
            let prim_id = tri4.prim_id[i];
            let last = prim_id < 0;

            let v0 = extract_vec3(&tri4.v0, i);
            let e1 = extract_vec3(&tri4.e1, i);
            let e2 = extract_vec3(&tri4.e2, i);

            let v1 = v0 - e1;
            let v2 = e2 + v0;

            let triangle = Triangle {
                v0: v0,
                v1: v1,
                v2: v2,
            };

            let tri_ref: &Triangle = triangles.alloc(triangle);
            prim_triangles.push(tri_ref);

            count += 1;
            if last {
                break 'outer;
            }
        }
        id += 1;
    }

    if count > 4 {
        println!("todo: more than 4 primitives in leaf node, sort this out!");
        count = 4;
    }

    let mut triangle_refs: [*const Triangle; 4] = [&DUMMY_TRIANGLE; 4];
    for i in 0..count {
        triangle_refs[i] = prim_triangles[i];
    }

    let node = LeafNode4 {
        real_count: count as i8,
        primitives: triangle_refs,
        bbox,
    };

    leaf_nodes.push(node);
    NodeId::Leaf((leaf_nodes.len() - 1) as i32)
}

fn write_inner_node(node8: Node8, env: &ConversionEnv) -> NodeId {
    let mut bbox = extract_bbox(&node8, 0);
    let mut count = 0;
    let mut child_nodes = [NodeId::None; 8];
    for i in 0..8 {
        let child = node8.child[i as usize];
        if child == 0 {
            break;
        }

        let child_bbox = extract_bbox(&node8, i);
        bbox = enclosing_bbox(&bbox, &child_bbox);

        let wrote_ref: NodeId;
        if child > 0 {
            let child_node8_id = child - 1;
            let child_node8 = env.node8vec[child_node8_id as usize];
            wrote_ref = write_inner_node(child_node8, env);
        } else {
            let child_tri4_id = child ^ -1;
            let mut borrow = env.leaf_nodes.borrow_mut();
            wrote_ref = write_leaf_node(child_bbox, child_tri4_id, &env.tri4vec, borrow.as_mut(), &env.triangles);
        }
        child_nodes[i as usize] = wrote_ref;
        count += 1;
    }

    let node = InnerNode8 {
        real_count: count,
        nodes: child_nodes,
        bbox: bbox,
    };
    env.inner_nodes.borrow_mut().push(node);
    return NodeId::Inner((env.inner_nodes.borrow().len() - 1) as i32);
}

const DUMMY_TRIANGLE: Triangle = Triangle {
    v0: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
    v1: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
    v2: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
};

fn extract_vec3(list: &[[f32; 4]; 3], i: usize) -> Vector3<f32> {
    Vector3 {
        x: list[0][i],
        y: list[1][i],
        z: list[2][i],
    }
}

fn extract_bbox(node8: &Node8, child: i32) -> BBox {
    let child = child as usize;
    BBox {
        min: Vector3 {
            x: node8.bounds[0][child],
            y: node8.bounds[2][child],
            z: node8.bounds[4][child],
        },
        max: Vector3 {
            x: node8.bounds[1][child],
            y: node8.bounds[3][child],
            z: node8.bounds[5][child],
        },
    }
}

fn read_i32<R>(reader: &mut R) -> i32
    where R: io::Read {
    let mut buf: [u8; 4] = [0; 4];
    let buf2 = &mut buf[..];
    reader.read_exact(buf2);
    let mut rdr = Cursor::new(buf);
    rdr.read_i32::<LittleEndian>().unwrap()
}

fn read_f32<R>(reader: &mut R) -> f32
    where R: io::Read {
    let mut buf: [u8; 4] = [0; 4];
    let buf2 = &mut buf[..];
    reader.read_exact(buf2);
    let mut rdr = Cursor::new(buf);
    rdr.read_f32::<LittleEndian>().unwrap()
}

fn read_buffer(reader: &mut BufReader<File>) -> Vec<u8> {
    let uncompressed_size = read_i32(reader);
    let compressed_size = read_i32(reader);

    let mut compressed_data: Vec<u8> = Vec::new();
    compressed_data.resize(compressed_size as usize, 0);
    {
        let target = &mut compressed_data[..];
        reader.read_exact(target);
    }

    let mut uncompressed_data: Vec<u8> = Vec::new();
    uncompressed_data.resize(uncompressed_size as usize, 0);

    {
        let k = lz4::decode_block(&mut compressed_data[..], &mut uncompressed_data);
    }

    uncompressed_data
}

fn reverse_endianness(oh_no: &mut Vec<u8>) {
    assert!(oh_no.len() % 4 == 0);

    let mut temp = [0_u8; 4];
    for i in 0..oh_no.len() / 4 {
        for j in 0..4 {
            temp[j] = oh_no[i + j];
        }
        for j in 0..4 {
            oh_no[i + j] = temp[3 - j];
        }
    }
}