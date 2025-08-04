use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::{Rc, Weak};
use macroquad::color::WHITE;
use macroquad::prelude::draw_text;
use macroquad::time::get_frame_time;
use common::math::vectors::Vec2;
use crate::effectors::Effector;
use crate::entity::{Entity, EntityType};

const RESTITUTION: f32 = 0.7;

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
                let (mut obj1, mut obj2) = (self.entities[i].borrow_mut(), self.entities[j].borrow_mut());
                let collision = obj1.get_collider().detect_collision(
                    &obj1.get_position(),
                    &obj2.get_collider(),
                    &obj2.get_position()
                );


                if let Some(collision) = collision {
                    let obj1_mass = obj1.get_mass();
                    let obj2_mass = obj2.get_mass();

                    // // let direction = (obj2.get_position() - obj1.get_position()).unit();
                    // // let distance = self.coords2screen(&(obj2.get_position() - obj1.get_position())).length();
                    // // let overlap = 20.0 - distance;
                    // let total_mass = obj1_mass + obj2_mass;
                    //
                    // *obj1.get_position_mut() = obj1.get_position() - (collision.direction * (collision.penetration * (obj2.get_mass() / total_mass)));
                    // *obj2.get_position_mut() = obj2.get_position() + (collision.direction * (collision.penetration * (obj1.get_mass() / total_mass)));
                    //
                    // let relative_velocity = obj1.get_velocity() - obj2.get_velocity();
                    // let velocity_along_normal = relative_velocity.dot(&collision.direction);
                    //
                    // if velocity_along_normal > 0.0 {
                    //     continue;
                    // }
                    //
                    // let mut impulse = -(1.0 + RESTITUTION) * velocity_along_normal;
                    // impulse /= 1.0 / obj1.get_mass() + 1.0 / obj2.get_mass();
                    //
                    // let impulse_vector = collision.direction * impulse;
                    //
                    // *obj1.get_velocity_mut() += impulse_vector / obj1_mass;
                    // *obj2.get_velocity_mut() += impulse_vector / obj2_mass;

                    match (obj1.get_type(), obj2.get_type()) {
                        (EntityType::Dynamic, EntityType::Dynamic) => {
                            let total_inv_mass = 1.0/obj1_mass + 1.0/obj2_mass;
                            let sep_a = (1.0/obj1_mass) / total_inv_mass;
                            let sep_b = (1.0/obj2_mass) / total_inv_mass;

                            *obj1.get_position_mut() -= collision.direction * collision.penetration * sep_a;
                            *obj2.get_position_mut() += collision.direction * collision.penetration * sep_b;
                        },
                        (EntityType::Static, EntityType::Dynamic) => {
                            *obj2.get_position_mut() += collision.direction * collision.penetration;
                        },
                        (EntityType::Dynamic, EntityType::Static) => {
                            *obj1.get_position_mut() -= collision.direction * collision.penetration;
                        },
                        (_, _) => {}
                    };

                    if !matches!(obj1.get_type(), EntityType::Static) || !matches!(obj2.get_type(), EntityType::Static) {
                        let inv_mass_a = match obj1.get_type() {
                            EntityType::Dynamic => 1.0 / obj1_mass,
                            EntityType::Static => 0.0
                        };

                        let inv_mass_b = match obj2.get_type() {
                            EntityType::Dynamic => 1.0 / obj2_mass,
                            EntityType::Static => 0.0
                        };

                        let total_inv_mass = inv_mass_a + inv_mass_b;
                        if total_inv_mass == 0.0 {
                            continue;
                        }

                        let relative_velocity = obj2.get_velocity() - obj1.get_velocity();
                        let velocity_along_normal = relative_velocity.dot(&collision.direction);

                        if velocity_along_normal > 0.0 {
                            continue;
                        }

                        let impulse_magnitude = -(1.0 + RESTITUTION) * velocity_along_normal / total_inv_mass;
                        let impulse = collision.direction * impulse_magnitude;

                        if !matches!(obj1.get_type(), EntityType::Static) {
                            *obj1.get_velocity_mut() -= impulse * inv_mass_a
                        }

                        if !matches!(obj2.get_type(), EntityType::Static) {
                            *obj2.get_velocity_mut() += impulse * inv_mass_b;
                        }
                    }
                }
                // let delta = obj1.get_position() - obj2.get_position();
                // let normal = delta.unit();
                // let relative_vel = obj1.velocity - obj2.velocity;
                //
                // let velocity_alog_normal = relative_vel.dot(&normal);
                //
                // if velocity_alog_normal <= 0.0 {
                //     let impulse_scalar = -(1.0 + RESTITUTION) * velocity_alog_normal / (1.0 / obj1.mass + 1.0 / obj2.mass);
                //     let impulse = normal * impulse_scalar;
                //     self.entities[i].velocity = self.entities[i].velocity + (impulse * (-1.0/self.entities[i].mass));
                //     self.entities[j].velocity = self.entities[j].velocity + (impulse * (1.0/self.entities[j].mass));
                // }
                //
                // let penetration = 20.0 - self.coords2screen(&delta).magnitude();
                //
                // let total_mass = self.entities[i].mass + self.entities[j].mass;
                // if total_mass > 0.0 {
                //     let correction = normal * (penetration / (total_mass*0.5));
                //     self.entities[i].position = self.entities[i].position + self.screen2coords(&(correction * -self.entities[j].mass));
                //     self.entities[j].position = self.entities[j].position + self.screen2coords(&(correction * self.entities[i].mass));
                // }
            }
        }

        for entity in &self.entities {
            entity.borrow_mut().render(self);
        }

        draw_text(&format!("{} entities", self.entities.iter().count()), 20.0, 50.0, 30.0, WHITE);
    }
}