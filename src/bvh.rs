extern crate either;

pub mod loader_rodent;
pub mod inner_arities;
pub mod leaf_arities;

use self::either::Either;
use crate::bbox::BBox;

#[derive(Clone, Copy, Debug)]
pub enum NodeId {
    Inner(i32),
    Leaf(i32),
    None
}

pub trait Node {
    fn bbox(&self) -> BBox;
}

pub trait InnerNode : Node {
    fn children_count(&self) -> i32;
    fn get_children(&self, children_ids: &mut [NodeId]);
}

pub trait LeafNode<P> : Node {
    fn primitives_count(&self) -> i32;
    fn get_primitives<'a>(&'a self, primitive_ids: &mut [&'a P]);
}

pub struct BvhTree<T, P>
    where T: InnerNode,
          P: LeafNode<P> {
    inner_nodes: Vec<T>,
    leaf_nodes: Vec<P>,
    root_node_id: NodeId,
}

impl<T, P> BvhTree<T, P>
    where T: InnerNode,
          P: LeafNode<P> {
    fn get_node<'a>(&'a self, id: NodeId) -> Either<&'a T, &'a P> {
        match (id) {
            NodeId::Inner(id) => Either::Left(self.inner_nodes.get(id as usize).expect(format!("Out of bounds id {}", id).as_str())),
            NodeId::Leaf(id) => Either::Right(self.leaf_nodes.get(id as usize).expect(format!("Out of bounds id {}", id).as_str())),
            NodeId::None => panic!("You can't lookup this!"),
        }
    }
}

//pub use self::inner_arities;
//pub use self::leaf_arities;