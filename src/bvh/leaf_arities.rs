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

    fn get_primitive<'a>(&'a self, n: usize) -> &'a P {
        return unsafe { &*(self.primitive) };
    }

    /*fn get_primitives<'a>(&'a self, primitive_ids: &mut [&'a P]) {
        primitive_ids[0] = self.primitive
    }*/
}

impl<P> Node for LeafNode1<P> {
    fn bbox(&self) -> BBox {
        self.bbox
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

    /*fn get_primitives<'a>(&'a self, primitive_ids: &mut [&'a P]) {
        for i in 0..self.real_count {
            primitive_ids[i as usize] = self.primitives[i as usize];
        }
    }*/

    fn get_primitive<'a>(&'a self, n: usize) -> &'a P {
        return unsafe { &*(self.primitives[n]) };
    }
}

impl<P> Node for LeafNode4<P> {
    fn bbox(&self) -> BBox {
        self.bbox
    }
}