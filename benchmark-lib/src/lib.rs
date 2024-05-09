use criterion::black_box;

pub mod bevy_ecs_bench;
pub mod game_objects_hash_bench;
pub mod game_objects_vec_bench;
pub mod specs_bench;
pub mod xanadu_bench;

#[repr(C)]
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    bytemuck::Pod,
    bytemuck::Zeroable,
    PartialEq,
    bevy_ecs::prelude::Component,
)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C)]
#[derive(
    Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq, bevy_ecs::prelude::Component,
)]
pub struct OtherData {
    pub data: [f64; 128],
}

impl Default for OtherData {
    fn default() -> Self {
        Self { data: [0.0; 128] }
    }
}

pub struct GameObject {
    pub id: u64,
    pub position: Option<Position>,
    pub other_data: Option<OtherData>,
}

pub fn shuffle_system(pos: &mut Position) {
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
