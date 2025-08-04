use std::cell::RefMut;
use crate::collisions::detection::Collision;
use crate::entity::{Entity, EntityType};

const RESTITUTION: f32 = 0.7;

pub fn handle_collision(mut obj1: RefMut<dyn Entity>, mut obj2: RefMut<dyn Entity>, collision: &Collision) {
    let obj1_mass = obj1.get_mass();
    let obj2_mass = obj2.get_mass();

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
    
    if matches!(obj1.get_type(), EntityType::Dynamic) || matches!(obj2.get_type(), EntityType::Dynamic) {
        apply_velocity_response(obj1, obj2, collision);
    }
}

fn apply_velocity_response(mut obj1: RefMut<dyn Entity>, mut obj2: RefMut<dyn Entity>, collision: &Collision) {
    let inv_mass_a = match obj1.get_type() {
        EntityType::Dynamic => 1.0 / obj1.get_mass(),
        EntityType::Static => 0.0
    };

    let inv_mass_b = match obj2.get_type() {
        EntityType::Dynamic => 1.0 / obj2.get_mass(),
        EntityType::Static => 0.0
    };

    let total_inv_mass = inv_mass_a + inv_mass_b;
    if total_inv_mass == 0.0 {
        return;
    }

    let relative_velocity = obj2.get_velocity() - obj1.get_velocity();
    let velocity_along_normal = relative_velocity.dot(&collision.direction);

    if velocity_along_normal > 0.0 {
        return;
    }

    let impulse_magnitude = -(1.0 + RESTITUTION) * velocity_along_normal / total_inv_mass;
    let impulse = collision.direction * impulse_magnitude;

    if matches!(obj1.get_type(), EntityType::Dynamic) {
        *obj1.get_velocity_mut() -= impulse * inv_mass_a
    }

    if matches!(obj2.get_type(), EntityType::Dynamic) {
        *obj2.get_velocity_mut() += impulse * inv_mass_b;
    }
}