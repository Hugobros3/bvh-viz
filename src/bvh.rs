extern crate either;

pub mod loader_rodent;
pub mod inner_arities;
pub mod leaf_arities;
pub mod traversal;

use self::either::Either;
use crate::bbox::BBox;
use std::marker::PhantomData;
use crate::geometry::Intersect;

#[derive(Clone, Copy, Debug)]
pub enum NodeId {
    Inner(i32),
    Leaf(i32),
    None,
}

pub trait Node {
    fn bbox(&self) -> BBox;
}

pub trait InnerNode: Node {
    fn children_count(&self) -> i32;
    fn get_children(&self, children_ids: &mut [NodeId]);
}

pub trait LeafNode<P>: Node
    where P: Intersect {
    fn primitives_count(&self) -> i32;
    //fn get_primitives<'a>(&'a self, primitive_ids: &mut [&'a P]);
    fn get_primitive<'a>(&'a self, n: usize) -> &'a P;
}

pub struct BvhTree<'a, P, I, L>
    where I: InnerNode,
          L: LeafNode<P>,
          P: Intersect {
    inner_nodes: &'a Vec<I>,
    leaf_nodes: &'a Vec<L>,
    root_node_id: NodeId,
    terrible: PhantomData<P>,
}

impl<'a, P, I, L> BvhTree<'a, P, I, L>
    where I: InnerNode,
          L: LeafNode<P>,
          P: Intersect {
    fn get_node(&self, id: NodeId) -> Either<&I, &L> {
        match (id) {
            NodeId::Inner(id) => Either::Left(self.inner_nodes.get(id as usize).expect(format!("Out of bounds id {}", id).as_str())),
            NodeId::Leaf(id) => Either::Right(self.leaf_nodes.get(id as usize).expect(format!("Out of bounds id {}", id).as_str())),
            NodeId::None => panic!("You can't lookup this!"),
        }
    }

    fn get_bbox(&self, id: NodeId) -> BBox {
        let node = self.get_node(id);
        match node {
            Either::Left(node) => { return node.bbox(); }
            Either::Right(node) => { return node.bbox(); }
        }
    }
}

//pub use self::inner_arities;
//pub use self::leaf_arities;