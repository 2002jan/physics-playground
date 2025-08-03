use common::math::vectors::Vec2;
use crate::collisions::colliders::{CircleBB, RectangleBB};

pub mod detection;
pub mod colliders;
pub mod response;

pub struct Collider {
    pub rel_pos: Vec2,
    pub bound_box: BoundBox
}

pub enum BoundBox {
    Circle(CircleBB),
    Rectangle(RectangleBB)
}