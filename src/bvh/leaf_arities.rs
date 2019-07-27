use crate::bvh::{LeafNode, Node};
use crate::bbox::BBox;

#[derive(Copy, Clone, Debug)]
pub struct LeafNode1<'a, P> {
    pub primitive: &'a P,
    pub bbox: BBox,
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

#[derive(Copy, Clone, Debug)]
pub struct LeafNode4<'a, P> {
    pub real_count: i8,
    pub primitives: [&'a P;4],
    pub bbox: BBox,
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