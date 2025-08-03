#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::effectors::forces::mouse_gravity::MouseGravity;
use crate::effectors::Effector;
use crate::entity::circle_entity::CircleEntity;
use crate::entity::rectangle_entity::RectangleEntity;
use crate::world::{EntityRef, World};
use common::math::vectors::Vec2;
use macroquad::color::{BLACK, WHITE};
use macroquad::input::{
    is_mouse_button_pressed, mouse_position, MouseButton
    ,
};
use macroquad::text::draw_text;
use macroquad::time::{get_fps, get_frame_time};
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use std::cell::RefCell;
use std::rc::Rc;

mod collisions;
pub mod effectors;
mod entity;
mod world;

#[macroquad::main("Physics Playground")]
async fn main() {
    let mut world = World::new(screen_width(), screen_height());
    let mut mouse_grav = MouseGravity::new(0.1);

    let mut i = 0;
    let mut fps = get_fps();

    loop {
        clear_background(BLACK);

        i += 1;

        if i % 100 == 0 {
            fps = get_fps();
            world.update_size(screen_width(), screen_height());
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let a = Vec2 { x, y };
            let e = CircleEntity::new(a.x, a.y);

            let rc: EntityRef = Rc::new(RefCell::new(e));

            world.add_entity(Rc::clone(&rc));
            mouse_grav.add_entity(Rc::downgrade(&rc))
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let (x, y) = mouse_position();
            let a = Vec2 { x, y };
            let e = RectangleEntity::new(a.x, a.y, 30.0, 30.0);


            let rc: EntityRef = Rc::new(RefCell::new(e));

            world.add_entity(Rc::clone(&rc));
            mouse_grav.add_entity(Rc::downgrade(&rc))
        }

        mouse_grav.update(get_frame_time());
        world.render_entities();

        draw_text(&format!("{} fps", fps), 20.0, 20.0, 30.0, WHITE);

        i %= 100;
        next_frame().await;
    }
}
