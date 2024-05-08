use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xanadu::ecs::dyn_pool::{Mut, World};

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
    pub position: Position,
    pub other_data: OtherData,
}

const NUM_ENTITIES: usize = 100_000;

fn shuffle_system(pos: &mut Position) {
    let tmp = pos.x;
    pos.x = pos.y;
    pos.y = pos.z;
    pos.z = tmp;
}

fn increment_system(pos: &mut Position) {
    pos.x += black_box(1.0);
    pos.y += black_box(2.0);
    pos.z += black_box(3.0);
}

fn shuffle_system_bevy1(mut query: bevy_ecs::prelude::Query<(&mut Position,)>) {
    for (mut pos,) in query.iter_mut() {
        shuffle_system(&mut pos);
    }
}

fn shuffle_system_bevy2(mut query: bevy_ecs::prelude::Query<(&mut Position,)>) {
    for (mut pos,) in query.iter_mut() {
        shuffle_system(&mut pos);
    }
}

fn increment_system_bevy(mut query: bevy_ecs::prelude::Query<(&mut Position,)>) {
    for (mut pos,) in query.iter_mut() {
        increment_system(&mut pos);
    }
}

fn benchmark_xanadu(c: &mut Criterion) {
    c.bench_function("xanadu", |b| {
        let mut world = World::builder()
            .register_component::<Position>()
            .register_component::<OtherData>()
            .build();

        for i in 0..NUM_ENTITIES {
            let entity = world.new_entity();
            world.attach_component(
                entity,
                Position {
                    x: black_box(i as f64 * 0.1),
                    y: black_box(i as f64 * 0.1),
                    z: black_box(i as f64 * 0.1),
                },
            );
            world.attach_component(entity, OtherData::default());
        }

        b.iter(|| {
            world.execute::<'_, Mut<Position>, _>(&shuffle_system);
            world.execute::<'_, Mut<Position>, _>(&increment_system);
            world.execute::<'_, Mut<Position>, _>(&shuffle_system);
        })
    });
}

fn benchmark_bevy_ecs(c: &mut Criterion) {
    use bevy_ecs::prelude::*;

    c.bench_function("bevy_ecs", |b| {
        let mut world = World::new();
        for i in 0..NUM_ENTITIES {
            world.spawn((
                Position {
                    x: black_box(i as f64 * 0.1),
                    y: black_box(i as f64 * 0.1),
                    z: black_box(i as f64 * 0.1),
                },
                OtherData::default(),
            ));
        }

        let mut schedule = Schedule::default();
        schedule.add_systems(
            increment_system_bevy
                .after(shuffle_system_bevy1)
                .before(shuffle_system_bevy2),
        );

        b.iter(|| schedule.run(&mut world))
    });
}

fn benchmark_game_objects_hash(c: &mut Criterion) {
    c.bench_function("game_objects_hash", |b| {
        let mut game_objects = HashMap::with_capacity(NUM_ENTITIES);
        for i in 0..NUM_ENTITIES {
            game_objects.insert(
                i as u64,
                GameObject {
                    id: i as u64,
                    position: Position {
                        x: black_box(i as f64 * 0.1),
                        y: black_box(i as f64 * 0.1),
                        z: black_box(i as f64 * 0.1),
                    },
                    other_data: OtherData::default(),
                },
            );
        }

        b.iter(|| {
            for game_object in game_objects.values_mut() {
                shuffle_system(&mut game_object.position);
            }
            for game_object in game_objects.values_mut() {
                increment_system(&mut game_object.position);
            }
            for game_object in game_objects.values_mut() {
                shuffle_system(&mut game_object.position);
            }
        })
    });
}

fn benchmark_game_objects_vec(c: &mut Criterion) {
    c.bench_function("game_objects_vec", |b| {
        let mut game_objects = Vec::with_capacity(NUM_ENTITIES);
        for i in 0..NUM_ENTITIES {
            game_objects.push(GameObject {
                id: i as u64,
                position: Position {
                    x: black_box(i as f64 * 0.1),
                    y: black_box(i as f64 * 0.1),
                    z: black_box(i as f64 * 0.1),
                },
                other_data: OtherData::default(),
            });
        }

        b.iter(|| {
            for game_object in game_objects.iter_mut() {
                shuffle_system(&mut game_object.position);
            }
            for game_object in game_objects.iter_mut() {
                increment_system(&mut game_object.position);
            }
            for game_object in game_objects.iter_mut() {
                shuffle_system(&mut game_object.position);
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_xanadu,
    benchmark_bevy_ecs,
    benchmark_game_objects_hash,
    benchmark_game_objects_vec,
);
criterion_main!(benches);
