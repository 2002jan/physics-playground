pub mod forces;

use std::any::Any;
use crate::world::WeakEntityRef;

pub trait Effector : Any {

    fn update(&mut self, delta: f32);

    fn get_entities(&self) -> &Vec<WeakEntityRef>;
    fn get_entities_mut(&mut self) -> &mut Vec<WeakEntityRef>;

    fn add_entity(&mut self, entity_ref: WeakEntityRef) {
        self.get_entities_mut().push(entity_ref);
    }

    fn cleanup(&mut self) {
        self.get_entities_mut().retain(|weak_entity| weak_entity.strong_count() > 0);
    }

    fn as_any(&self) -> &dyn Any;
}