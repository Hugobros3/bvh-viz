use crate::bvh::{LeafNode, Node};
use crate::bbox::BBox;
use crate::geometry::Intersect;

#[derive(Copy, Clone, Debug)]
pub struct LeafNode1<P> {
    pub primitive: *const P,
    pub bbox: BBox,
}

impl<P> LeafNode<P> for LeafNode1<P> where
P : Intersect {
    fn primitives_count(&self) -> i32 {
        1
    }

    fn get_primitive(&self, n: usize) -> &P {
        return unsafe { &*(self.primitive) };
    }
}

impl<P> Node for LeafNode1<P> {
    fn bbox(&self) -> &BBox {
        &self.bbox
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LeafNode4<P> {
    pub real_count: i8,
    pub primitives: [*const P;4],
    pub bbox: BBox,
}

impl<P> LeafNode<P> for LeafNode4<P>where
    P : Intersect  {
    fn primitives_count(&self) -> i32 {
        self.real_count as i32
    }

    fn get_primitive(&self, n: usize) -> &P {
        return unsafe { &*(self.primitives[n]) };
    }
}

unsafe impl<P> Sync for LeafNode4<P> {}

impl<P> Node for LeafNode4<P> {
    fn bbox(&self) -> &BBox {
        &self.bbox
    }
}