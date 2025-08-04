use crate::collisions::colliders::CircleBB;
use crate::collisions::BoundBox::Circle;
use crate::collisions::Collider;
use crate::entity::{Entity, EntityType};
use crate::world::World;
use common::math::vectors::Vec2;
use macroquad::color::GREEN;
use macroquad::shapes::draw_circle;

pub struct CircleEntity {
    pub position: Vec2,
    force: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub collider: Collider,
    pub entity_type: EntityType
}

impl CircleEntity {
    pub fn new(x: f32, y: f32) -> Self{
        Self {
            position: Vec2{x,y},
            force: Vec2::zeros(),
            velocity: Vec2::zeros(),
            mass: 2.0,
            collider: Collider {
                rel_pos: Vec2::zeros(),
                bound_box: Circle(CircleBB {
                  radius: 10.0
                })
            },
            entity_type: EntityType::Dynamic
        }
    }
}

impl Entity for CircleEntity {
    fn render(&self, _world: &World) {
        draw_circle(self.position.x, self.position.y, 10.0, GREEN)
    }

    fn get_position(&self) -> Vec2 {
        self.position.clone()
    }

    fn get_position_mut(&mut self) -> &mut Vec2 {
        &mut self.position
    }

    fn get_velocity(&self) -> Vec2 {
        self.velocity
    }

    fn get_velocity_mut(&mut self) -> &mut Vec2 {
        &mut self.velocity
    }

    fn get_force(&self) -> Vec2 {
        self.force
    }

    fn get_force_mut(&mut self) -> &mut Vec2 {
        &mut self.force
    }

    fn get_collider(&self) -> &Collider {
        &self.collider
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_type(&self) -> &EntityType {
        &self.entity_type
    }
}