use std::collections::HashMap;

use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};
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

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark");
    for i in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::new("xanadu", i), i, |b, i| {
            let mut world = setup_xanadu(*i);
            b.iter(|| benchmark_xanadu(&mut world));
        });
        group.bench_with_input(BenchmarkId::new("bevy_ecs", i), i, |b, i| {
            let (mut world, mut schedule) = setup_bevy_ecs(*i);
            b.iter(|| benchmark_bevy_ecs(&mut world, &mut schedule));
        });
        group.bench_with_input(BenchmarkId::new("game_objects_hash", i), i, |b, i| {
            let mut game_objects = setup_game_objects_hash(*i);
            b.iter(|| benchmark_game_objects_hash(&mut game_objects));
        });
        group.bench_with_input(BenchmarkId::new("game_objects_vec", i), i, |b, i| {
            let mut game_objects = setup_game_objects_vec(*i);
            b.iter(|| benchmark_game_objects_vec(&mut game_objects));
        });
    }
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

fn setup_xanadu(n: usize) -> World {
    let mut world = World::builder()
        .register_component::<Position>()
        .register_component::<OtherData>()
        .build();

    for i in 0..n {
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

    world
}

fn benchmark_xanadu(world: &mut World) {
    world.execute::<'_, Mut<Position>, _>(&shuffle_system);
    world.execute::<'_, Mut<Position>, _>(&increment_system);
    world.execute::<'_, Mut<Position>, _>(&shuffle_system);
}

fn setup_bevy_ecs(n: usize) -> (bevy_ecs::prelude::World, bevy_ecs::prelude::Schedule) {
    use bevy_ecs::prelude::*;

    let mut world = bevy_ecs::prelude::World::new();
    for i in 0..n {
        world.spawn((Position {
            x: black_box(i as f64 * 0.1),
            y: black_box(i as f64 * 0.1),
            z: black_box(i as f64 * 0.1),
        },));
    }

    let mut schedule = Schedule::default();
    schedule.add_systems(
        increment_system_bevy
            .after(shuffle_system_bevy1)
            .before(shuffle_system_bevy2),
    );

    (world, schedule)
}

fn benchmark_bevy_ecs(
    mut world: &mut bevy_ecs::prelude::World,
    schedule: &mut bevy_ecs::prelude::Schedule,
) {
    schedule.run(&mut world);
}

fn setup_game_objects_hash(n: usize) -> HashMap<u64, GameObject> {
    let mut game_objects = HashMap::with_capacity(n);
    for i in 0..n {
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
    game_objects
}

fn benchmark_game_objects_hash(game_objects: &mut HashMap<u64, GameObject>) {
    for game_object in game_objects.values_mut() {
        shuffle_system(&mut game_object.position);
    }
    for game_object in game_objects.values_mut() {
        increment_system(&mut game_object.position);
    }
    for game_object in game_objects.values_mut() {
        shuffle_system(&mut game_object.position);
    }
}

fn setup_game_objects_vec(n: usize) -> Vec<GameObject> {
    let mut game_objects = Vec::with_capacity(n);
    for i in 0..n {
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
    game_objects
}

fn benchmark_game_objects_vec(game_objects: &mut Vec<GameObject>) {
    for game_object in game_objects.iter_mut() {
        shuffle_system(&mut game_object.position);
    }
    for game_object in game_objects.iter_mut() {
        increment_system(&mut game_object.position);
    }
    for game_object in game_objects.iter_mut() {
        shuffle_system(&mut game_object.position);
    }
}

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
