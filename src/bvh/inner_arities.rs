use crate::bvh::{NodeId, InnerNode, Node};
use crate::bbox::BBox;

pub struct InnerNode2 {
    left: NodeId,
    right: NodeId,
    bbox: BBox,
}

impl InnerNode for InnerNode2 {
    fn children_count(&self) -> i32 {
        2
    }

    fn get_children(&self, children_ids: &mut [NodeId]) {
        children_ids[0] = self.left;
        children_ids[1] = self.right;
    }
}

impl Node for InnerNode2 {
    fn bbox(&self) -> BBox {
        self.bbox
    }
}

pub struct InnerNode8 {
    real_count: i8,
    nodes: [NodeId;8],
    bbox: BBox,
}

impl InnerNode for InnerNode8 {
    fn children_count(&self) -> i32 {
        self.real_count as i32
    }

    fn get_children(&self, children_ids: &mut [NodeId]) {
        for i in 0..self.real_count {
            children_ids[i as usize] = self.nodes[i as usize];
        }
    }
}

impl Node for InnerNode8 {
    fn bbox(&self) -> BBox {
        self.bbox
    }
}