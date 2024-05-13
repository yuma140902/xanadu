use std::cell::RefMut;

pub mod bevy_ecs_bench;
pub mod game_objects_hash_bench;
pub mod game_objects_vec_bench;
pub mod specs_bench;
pub mod xanadu_exclusive_bench;
pub mod xanadu_refcell_bench;

#[derive(Debug, Clone, PartialEq, bevy_ecs::prelude::Component)]
pub struct Id(usize);

#[derive(Debug, Clone, PartialEq, bevy_ecs::prelude::Component)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, bevy_ecs::prelude::Component)]
pub struct OtherData {
    pub data: [f64; 128],
}

impl Default for OtherData {
    fn default() -> Self {
        Self { data: [0.0; 128] }
    }
}

pub struct GameObject {
    pub id: usize,
    pub position: Option<Position>,
    pub other_data: Option<OtherData>,
}

pub fn shuffle_system(pos: &mut Position) {
    let tmp = pos.x;
    pos.x = pos.y;
    pos.y = pos.z;
    pos.z = tmp;
}

pub fn shuffle_system_refmut(pos: &mut RefMut<Position>) {
    let tmp = pos.x;
    pos.x = pos.y;
    pos.y = pos.z;
    pos.z = tmp;
}

pub fn increment_system(pos: &mut Position) {
    pos.x += black_box(1.0);
    pos.y += black_box(2.0);
    pos.z += black_box(3.0);
}

pub fn increment_system_refmut(pos: &mut RefMut<Position>) {
    pos.x += black_box(1.0);
    pos.y += black_box(2.0);
    pos.z += black_box(3.0);
}

#[inline(always)]
#[cfg(not(target_arch = "wasm32"))]
pub fn black_box<T>(x: T) -> T {
    criterion::black_box(x)
}

#[inline(always)]
#[cfg(target_arch = "wasm32")]
pub fn black_box<T>(x: T) -> T {
    x
}
