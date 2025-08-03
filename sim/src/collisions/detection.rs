use std::ops::Neg;
use crate::collisions::colliders::{CircleBB, RectangleBB};
use crate::collisions::{BoundBox, Collider};
use BoundBox::{Circle, Rectangle};
use common::math::vectors::Vec2;

pub struct Collision {
    pub direction: Vec2,
    pub penetration: f32,
}

impl Neg for Collision {
    type Output = Collision;

    fn neg(mut self) -> Self::Output {
        self.direction = -self.direction;
        self
    }
}

impl Collider {
    pub fn detect_collision(&self, pos: &Vec2, other_collider: &Collider, other_pos: &Vec2) -> Option<Collision> {
        let pos_a = *pos + self.rel_pos;
        let pos_b = *other_pos + other_collider.rel_pos;

        match (&self.bound_box, &other_collider.bound_box) {
            // Same types
            (Circle(a), Circle(b)) => detect_circle_circle(a, pos_a, b, pos_b),
            (Rectangle(a), Rectangle(b)) => detect_rectangle_rectangle(a, pos_a, b, pos_b),

            // Mixed cases
            (Circle(a), Rectangle(b)) => {
                detect_circle_rectangle(a, pos_a, b, pos_b)
            }
            (Rectangle(a), Circle(b)) => {
                Some(-detect_circle_rectangle(b, pos_b, a, pos_a)?)
            }
        }
    }
}

// Same types detection functions
fn detect_circle_circle(a: &CircleBB, a_pos: Vec2, b: &CircleBB, b_pos: Vec2) -> Option<Collision> {
    let dist = a_pos.dist(&b_pos);

    if dist > a.radius + b.radius {
        return None;
    }

    let direction = (b_pos - a_pos).unit();
    let penetration = (a.radius + b.radius) - dist;

    Some(Collision { direction, penetration })
}

fn detect_rectangle_rectangle(a: &RectangleBB, a_pos: Vec2, b: &RectangleBB, b_pos: Vec2) -> Option<Collision> {
    if a_pos.x + a.width < b_pos.x
        || a_pos.x > b_pos.x + b.width
        || a_pos.y + a.height < b_pos.y
        || a_pos.y > b_pos.y + b.height {
        return None;
    }

    let overlap_x = f32::min(a_pos.x + a.width, b_pos.x + b.width) - f32::max(a_pos.x, b_pos.x);
    let overlap_y = f32::min(a_pos.y + a.height, b_pos.y + b.height) - f32::max(a_pos.y, b_pos.y);

    if overlap_x < overlap_y {
        Some(Collision {
            penetration: overlap_x,
            direction: Vec2::new(
                if a_pos.x < b_pos.x { 1.0 } else { -1.0 },
                0.0,
            ),
        })
    } else {
        Some(Collision {
            penetration: overlap_y,
            direction: Vec2::new(
                0.0,
                if a_pos.y < b_pos.y { 1.0 } else { -1.0 },
            ),
        })
    }
}

// Mixed types detection functions
fn detect_circle_rectangle(a: &CircleBB, a_pos: Vec2, b: &RectangleBB, b_pos: Vec2) -> Option<Collision> {
    let mut closest_x = a_pos.x;
    if a_pos.x < b_pos.x {
        closest_x = b_pos.x
    } else if a_pos.x > b_pos.x + b.width {
        closest_x = b_pos.x + b.width
    }

    let mut closest_y = a_pos.y;
    if a_pos.y < b_pos.y {
        closest_y = b_pos.y
    } else if a_pos.y > b_pos.y + b.height {
        closest_y = b_pos.y + b.height
    }

    let mut d = a_pos - Vec2::new(closest_x, closest_y);
    d = -d;
    let mut distance = d.length();

    if distance > a.radius {
        return None;
    }

    if distance == 0.0 {
        let dx_edge = f32::min(a_pos.x - b_pos.x, b_pos.x + b.width - a_pos.x);
        let dy_edge = f32::min(a_pos.y - b_pos.y, b_pos.y + b.height - a_pos.y);

        d = if dx_edge < dy_edge {
            Vec2::new(
                if a_pos.x < b_pos.x + (b.width / 2.0) { 1.0 } else { -1.0 },
                0.0,
            )
        } else {
            Vec2::new(
                0.0,
                if a_pos.y < b_pos.y + (b.height / 2.0) { 1.0 } else { -1.0 },
            )
        };

        distance = f32::min(dx_edge, dy_edge);
    }

    Some(Collision {
        penetration: a.radius - distance,
        direction: d.unit(),
    })
}
