use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::{Rc, Weak};
use macroquad::color::WHITE;
use macroquad::prelude::draw_text;
use macroquad::time::get_frame_time;
use common::math::vectors::Vec2;
use crate::collisions::response::handle_collision;
use crate::effectors::Effector;
use crate::entity::{Entity, EntityType};

pub type EntityRef = Rc<RefCell<dyn Entity>>;
pub type WeakEntityRef = Weak<RefCell<dyn Entity>>;

pub struct World {
    size: Vec2,
    pub entities: Vec<EntityRef>,
    pub effectors: LinkedList<Box<dyn Effector>>
}

impl World {
    pub fn new(height: f32, width: f32) -> Self {
        World {
            size: Vec2::new(width, height),
            entities: Vec::new(),
            effectors: LinkedList::new(),
        }
    }

    pub fn update_size(&mut self, x: f32, y: f32) {
        self.size = Vec2::new(x, y);
    }

    pub fn add_entity(&mut self, entity: EntityRef) {
        self.entities.push(entity)
    }

    pub fn add_effector(&mut self, effector: Box<dyn Effector>) {
        self.effectors.push_back(effector);
    }

    pub fn update(&mut self, delta: f32) {
        for effector in &mut self.effectors {
            effector.update(delta);
        }
    }

    pub fn render_entities(&mut self) {
        self.entities.retain(|x| {
            let x = x.borrow();
            let pos = x.get_position();

            if matches!(x.get_type(), EntityType::Static) {
                return true;
            }

            (pos.x >= 0.0 && pos.x <= self.size.x) && (pos.y >= 0.0 && pos.y <= self.size.y)
        });

        let delta_time = get_frame_time();

        for entity in &mut self.entities {
            entity.borrow_mut().update(delta_time);
        }

        for i in 0..self.entities.len() {
            for j in (i + 1)..self.entities.len() {
                let (obj1, obj2) = (self.entities[i].borrow_mut(), self.entities[j].borrow_mut());
                let collision = obj1.get_collider().detect_collision(
                    &obj1.get_position(),
                    &obj2.get_collider(),
                    &obj2.get_position()
                );


                if let Some(collision) = collision {
                    handle_collision(obj1, obj2, &collision);
                }
            }
        }

        for entity in &self.entities {
            entity.borrow_mut().render(self);
        }

        draw_text(&format!("{} entities", self.entities.iter().count()), 20.0, 50.0, 30.0, WHITE);
    }
}