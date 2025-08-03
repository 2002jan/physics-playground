use std::any::Any;
use crate::effectors::Effector;
use crate::world::WeakEntityRef;
use macroquad::input::mouse_position;
use common::math::vectors::Vec2;

pub struct MouseGravity {
    pub entities: Vec<WeakEntityRef>,
    force: f32
}

impl MouseGravity {
    pub fn new(force: f32) -> Self {
        Self {
            entities: vec![],
            force,
        }
    }
}

impl Effector for MouseGravity {
    fn update(&mut self, _delta: f32) {
        let (x, y) = mouse_position();
        let mouse_positon = Vec2::new(x, y);

        self.entities.retain(|entity| {
            if let Some(entity) = entity.upgrade() {
                let mut entity = entity.borrow_mut();

                let f = mouse_positon - entity.get_position();
                let r = f.length();
                let f = if r > 0.0 {f.unit() * r.powi(2)} else {Vec2::zeros()} ;
                let f = f * self.force;

                entity.apply_force(f);

                true
            } else {
                false
            }
        });
    }

    fn get_entities(&self) -> &Vec<WeakEntityRef> {
        &self.entities
    }

    fn get_entities_mut(&mut self) -> &mut Vec<WeakEntityRef> {
        &mut self.entities
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
