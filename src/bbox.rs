use cgmath::{vec3, Vector3, InnerSpace};
use crate::ray::Ray;
use cgmath::num_traits::float::FloatCore;
use crate::geometry::Intersect;

#[derive(Copy, Clone, Debug)]
pub struct BBox {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

pub fn enclosing_bbox(left: &BBox, right: &BBox) -> BBox {
    let min = vec3_min(left.min, right.min);
    let max = vec3_max(left.max, right.max);
    BBox {
        min: min,
        max: max,
    }
}

fn min<S: PartialOrd>(left: S, right: S) -> S {
    if left < right { left } else { right }
}

pub fn vec3_min<S>(left: Vector3<S>, right: Vector3<S>) -> Vector3<S>
    where S: PartialOrd {
    Vector3 {
        x: min(left.x, right.x),
        y: min(left.y, right.y),
        z: min(left.z, right.z),
    }
}

fn max<S: PartialOrd>(left: S, right: S) -> S {
    if left > right { left } else { right }
}

pub fn vec3_max<S>(left: Vector3<S>, right: Vector3<S>) -> Vector3<S>
    where S: PartialOrd {
    Vector3 {
        x: max(left.x, right.x),
        y: max(left.y, right.y),
        z: max(left.z, right.z),
    }
}

fn sign(v: Vector3<f32>) -> Vector3<f32> {
    Vector3 { x: v.x.signum(), y: v.y.signum(), z: v.y.signum() }
}

impl Intersect for BBox {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let max_dist: f32 = f32::max_value();
        let min_dist: f32 = 0.0;

        //let sign = sign(ray.inverse_direction);
        let sign_x = ray.inverse_direction.x < 0.0;
        let sign_y = ray.inverse_direction.y < 0.0;
        let sign_z = ray.inverse_direction.z < 0.0;

        let bbox = if sign_x { self.max } else { self.min };
        let mut txmin = (bbox.x - ray.origin.x) * ray.inverse_direction.x;
        let bbox = if sign_x { self.min } else { self.max };
        let mut txmax = (bbox.x - ray.origin.x) * ray.inverse_direction.x;

        let bbox = if sign_y { self.max } else { self.min };
        let mut tymin = (bbox.y - ray.origin.y) * ray.inverse_direction.y;
        let bbox = if sign_y { self.min } else { self.max };
        let mut tymax = (bbox.y - ray.origin.y) * ray.inverse_direction.y;

        if txmin > tymax || tymin > txmax {
            return Option::None;
        }

        if tymin > txmin {
            txmin = tymin;
        }
        if tymax < txmax {
            txmax = tymax;
        }

        let bbox = if sign_z { self.max } else { self.min };
        let mut tzmin = (bbox.z - ray.origin.z) * ray.inverse_direction.z;
        let bbox = if sign_z { self.min } else { self.max };
        let mut tzmax = (bbox.z - ray.origin.z) * ray.inverse_direction.z;

        if txmin > tzmax || tzmin > txmax {
            return Option::None;
        }

        if tzmin > txmin {
            txmin = tzmin;
        }
        if tzmax < txmax {
            txmax = tzmax;
        }

        if txmin < max_dist && txmax > min_dist {
            return Option::Some(txmin)
        }

        return Option::None;
    }
}