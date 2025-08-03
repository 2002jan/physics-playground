pub mod circle_entity;
pub mod rectangle_entity;

use crate::collisions::Collider;
use crate::world::World;
use common::math::vectors::Vec2;

pub enum EntityType {
    Dynamic,
    Static
}

pub trait Entity {
    fn render(&self, world: &World);

    fn apply_force(&mut self, force: Vec2) {
        *self.get_force_mut() += force;
    }
    
    fn update(&mut self, delta_time: f32) {
        let vel = self.get_velocity();
        let d = (vel * -1.0) * 0.01;
        let f = d + self.get_force();

        let a = f / self.get_mass();

        *self.get_position_mut() += vel * delta_time + 0.5 * a * delta_time * delta_time;
        *self.get_velocity_mut() += a * delta_time;

        *self.get_force_mut() = Vec2::zeros();
    }

    fn get_position(&self) -> Vec2;
    fn get_position_mut(&mut self) -> &mut Vec2;
    fn get_velocity(&self) -> Vec2;
    fn get_velocity_mut(&mut self) -> &mut Vec2;
    fn get_force(&self) -> Vec2;
    fn get_force_mut(&mut self) -> &mut Vec2;
    fn get_collider(&self) -> &Collider;
    fn get_mass(&self) -> f32;
    fn get_type(&self) -> &EntityType;
}
