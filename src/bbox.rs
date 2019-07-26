use cgmath::{vec3, Vector3, InnerSpace};

#[derive(Copy, Clone)]
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
    if (left < right)
    { left } else { right }
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
    if (left > right)
    { left } else { right }
}

pub fn vec3_max<S>(left: Vector3<S>, right: Vector3<S>) -> Vector3<S>
    where S: PartialOrd {
    Vector3 {
        x: max(left.x, right.x),
        y: max(left.y, right.y),
        z: max(left.z, right.z),
    }
}