use std::cell::RefMut;

use crate::black_box;

pub mod bevy_ecs_bench;
pub mod game_objects_hash_bench;
pub mod game_objects_vec_bench;
pub mod specs_bench;
pub mod xanadu_bench;

#[derive(Debug, Clone, PartialEq, Eq, bevy_ecs::prelude::Component)]
pub struct Id(usize);

#[derive(Debug, Clone, PartialEq, bevy_ecs::prelude::Component)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, PartialEq, bevy_ecs::prelude::Component)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct GameObject {
    pub id: usize,
    pub position: Option<Position>,
    pub velocity: Option<Velocity>,
}

pub fn apply_velocity_system(pos: &mut Position, vel: &Velocity) {
    pos.x += vel.x;
    pos.y += vel.y;
    pos.z += vel.z;
}

pub fn apply_velocity_system_refcell(pos: &mut RefMut<Position>, vel: &mut RefMut<Velocity>) {
    pos.x += vel.x;
    pos.y += vel.y;
    pos.z += vel.z;
}

pub fn decay_velocity_system(vel: &mut Velocity) {
    vel.x *= black_box(0.9);
    vel.y *= black_box(0.9);
    vel.z *= black_box(0.9);
}

pub fn decay_velocity_system_refcell(vel: &mut RefMut<Velocity>) {
    vel.x *= black_box(0.9);
    vel.y *= black_box(0.9);
    vel.z *= black_box(0.9);
}
