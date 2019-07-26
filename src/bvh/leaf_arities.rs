use crate::bvh::{LeafNode, Node};
use crate::bbox::BBox;

pub struct LeafNode1<'a, P> {
    primitive: &'a P,
    bbox: BBox,
}

impl<P> LeafNode<P> for LeafNode1<'_, P> {
    fn primitives_count(&self) -> i32 {
        1
    }

    fn get_primitives<'a>(&'a self, primitive_ids: &mut [&'a P]) {
        primitive_ids[0] = self.primitive
    }
}

impl<P> Node for LeafNode1<'_, P> {
    fn bbox(&self) -> BBox {
        self.bbox
    }
}

pub struct LeafNode4<'a, P> {
    real_count: i8,
    primitives: [&'a P;4],
    bbox: BBox,
}

impl<P> LeafNode<P> for LeafNode4<'_, P> {
    fn primitives_count(&self) -> i32 {
        self.real_count as i32
    }

    fn get_primitives<'a>(&'a self, primitive_ids: &mut [&'a P]) {
        for i in 0..self.real_count {
            primitive_ids[i as usize] = self.primitives[i as usize];
        }
    }
}

impl<P> Node for LeafNode4<'_, P> {
    fn bbox(&self) -> BBox {
        self.bbox
    }
}