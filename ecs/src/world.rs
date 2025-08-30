use crate::entity::Entity;

pub struct World {
    next_id: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_id: 0
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;

        Entity::new(id)
    }
}