use crate::bvh::{NodeId, BvhTree, InnerNode, LeafNode};
use crate::ray::{Ray, HitPoint};
use cgmath::num_traits::float::FloatCore;
use super::either::Either;
use crate::geometry::Intersect;

struct StackElem {
    node_id: NodeId,
    t_min: f32,
}

impl<'a, P, I, L> BvhTree<'a, P, I, L>
    where I: InnerNode,
          L: LeafNode<P>,
          P: Intersect {
    pub fn trace(&self, ray: &mut Ray, any_hit: bool) -> bool {
        let mut stack: Vec<StackElem> = vec![StackElem { node_id: self.root_node_id, t_min: 0.0 }];
        let mut closest_hit = f32::max_value();

        while (!stack.is_empty()) {
            ray.steps += 1;

            let elem = stack.remove(stack.len() - 1);
            if elem.t_min > ray.t_max {
                continue;
            }

            let node = self.get_node(elem.node_id);
            match node {
                Either::Left(inner_node) => {
                    let mut children = [NodeId::None; 8];
                    inner_node.get_children(&mut children);

                    //let mut sorted: Vec<(f32, NodeId)> = Vec::new();
                    let mut sorted = [(0.0,NodeId::None); 8];
                    let mut scount = 0;

                    for child in 0..inner_node.children_count() {
                        let child_id = children[child as usize];
                        let bbox = self.get_bbox(child_id);

                        let intersection = bbox.intersect(ray);
                        if let Option::Some(distance) = intersection {
                            let mut insert: usize = 0;
                            for i in 0..scount {
                                if distance > sorted.get(i).unwrap().0 {
                                    break;
                                }
                                insert += 1;
                            }

                            //sorted.insert(insert, (distance, child_id));
                            /*if(insert == scount) {
                                sorted[insert] = (distance, child_id);
                            } else if (insert < scount ){
                                for i in (insert..(scount)).rev() {
                                    sorted[i + 1] = sorted[i];
                                }
                                sorted[insert] = (distance, child_id);
                            } else {
                                panic!("impossible");
                            }*/
                            sorted[scount] = (distance, child_id);
                            scount+=1;
                        }
                    }

                    for e in 0..scount {
                        let element = sorted[e];
                        stack.push(StackElem { node_id: element.1, t_min: element.0 })
                    }

                }
                Either::Right(leaf_node) => {
                    for i in 0..leaf_node.primitives_count() {
                        let primitive = leaf_node.get_primitive(i as usize);
                        let intersection = primitive.intersect(ray);
                        if let Option::Some(distance) = intersection {
                            let final_position = ray.origin + (ray.direction * distance);
                            let hit_point = HitPoint { position: final_position} ;
                            if any_hit {
                                ray.hit_point = Option::Some(hit_point);
                                ray.t_max = distance;
                                return true
                            } else {
                                if distance < closest_hit {
                                    closest_hit = distance;
                                    ray.hit_point = Option::Some(hit_point);
                                    ray.t_max = distance;
                                }
                            }
                        }
                    }
                }
            }
        }

        return closest_hit != f32::max_value();
    }
}