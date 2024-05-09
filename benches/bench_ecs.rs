use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};

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

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark");
    for i in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::new("xanadu", i), i, |b, i| {
            let mut world = xanadu_bench::setup(*i);
            b.iter(|| xanadu_bench::benchmark(&mut world));
        });
        group.bench_with_input(BenchmarkId::new("bevy_ecs", i), i, |b, i| {
            let (mut world, mut schedule) = bevy_ecs_bench::setup(*i);
            b.iter(|| bevy_ecs_bench::benchmark(&mut world, &mut schedule));
        });
        group.bench_with_input(BenchmarkId::new("specs", i), i, |b, i| {
            let (mut world, mut dispatcher) = specs_bench::setup(*i);
            b.iter(|| specs_bench::benchmark(&mut world, &mut dispatcher));
        });
        group.bench_with_input(BenchmarkId::new("game_objects_vec", i), i, |b, i| {
            let mut game_objects = game_objects_vec_bench::setup(*i);
            b.iter(|| game_objects_vec_bench::benchmark(&mut game_objects));
        });
        group.bench_with_input(BenchmarkId::new("game_objects_hash", i), i, |b, i| {
            let mut game_objects = game_objects_hash_bench::setup(*i);
            b.iter(|| game_objects_hash_bench::benchmark(&mut game_objects));
        });
    }
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

mod xanadu_bench {
    use criterion::black_box;
    use xanadu::ecs::dyn_pool::{Mut, World};

    use crate::{increment_system, shuffle_system, OtherData, Position};

    pub fn setup(n: usize) -> World {
        let mut world = World::builder()
            .register_component::<Position>()
            .register_component::<OtherData>()
            .build();

        for i in 0..n {
            let entity = world.new_entity();
            if i % 4 != 0 {
                world.attach_component(
                    entity,
                    Position {
                        x: black_box(i as f64 * 0.1),
                        y: black_box(i as f64 * 0.1),
                        z: black_box(i as f64 * 0.1),
                    },
                );
            }
            if i % 3 == 0 {
                world.attach_component(entity, OtherData::default());
            }
        }

        world
    }

    pub fn benchmark(world: &mut World) {
        world.execute::<'_, Mut<Position>, _>(&shuffle_system);
        world.execute::<'_, Mut<Position>, _>(&increment_system);
        world.execute::<'_, Mut<Position>, _>(&shuffle_system);
    }
}

mod bevy_ecs_bench {
    use bevy_ecs::prelude::*;
    use criterion::black_box;

    use crate::{increment_system, shuffle_system, OtherData, Position};

    pub fn setup(n: usize) -> (World, Schedule) {
        let mut world = World::new();
        for i in 0..n {
            if i % 4 != 0 && i % 3 == 0 {
                world.spawn((
                    Position {
                        x: black_box(i as f64 * 0.1),
                        y: black_box(i as f64 * 0.1),
                        z: black_box(i as f64 * 0.1),
                    },
                    OtherData::default(),
                ));
            } else if i % 4 != 0 {
                world.spawn((Position {
                    x: black_box(i as f64 * 0.1),
                    y: black_box(i as f64 * 0.1),
                    z: black_box(i as f64 * 0.1),
                },));
            } else if i % 3 == 0 {
                world.spawn((OtherData::default(),));
            } else {
                world.spawn(());
            }
        }

        let mut schedule = Schedule::default();
        schedule.add_systems(
            increment_system_bevy
                .after(shuffle_system_bevy1)
                .before(shuffle_system_bevy2),
        );

        (world, schedule)
    }

    pub fn benchmark(world: &mut World, schedule: &mut Schedule) {
        schedule.run(world);
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
}

mod specs_bench {
    use criterion::black_box;
    use specs::prelude::*;

    use crate::{increment_system, shuffle_system, OtherData, Position};

    impl Component for Position {
        type Storage = VecStorage<Self>;
    }

    impl Component for OtherData {
        type Storage = VecStorage<Self>;
    }

    struct IncrementSystem;

    impl<'a> System<'a> for IncrementSystem {
        type SystemData = WriteStorage<'a, Position>;

        fn run(&mut self, mut positions: Self::SystemData) {
            for pos in (&mut positions).join() {
                increment_system(pos);
            }
        }
    }

    struct ShuffleSystem;

    impl<'a> System<'a> for ShuffleSystem {
        type SystemData = WriteStorage<'a, Position>;
        fn run(&mut self, mut positions: Self::SystemData) {
            for pos in (&mut positions).join() {
                shuffle_system(pos);
            }
        }
    }

    pub fn setup(n: usize) -> (World, Dispatcher<'static, 'static>) {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<OtherData>();

        let mut dispatcher = DispatcherBuilder::new()
            .with(ShuffleSystem, "shuffle_system_1", &[])
            .with(IncrementSystem, "increment_system", &["shuffle_system_1"])
            .with(ShuffleSystem, "shuffle_system_2", &["increment_system"])
            .build();
        dispatcher.setup(&mut world);

        for i in 0..n {
            let mut entity_builder = world.create_entity();
            if i % 4 != 0 {
                entity_builder = entity_builder.with(Position {
                    x: black_box(i as f64 * 0.1),
                    y: black_box(i as f64 * 0.1),
                    z: black_box(i as f64 * 0.1),
                });
            }
            if i % 3 == 0 {
                entity_builder = entity_builder.with(OtherData::default());
            }
            entity_builder.build();
        }

        (world, dispatcher)
    }

    pub fn benchmark(world: &mut World, dispatcher: &mut Dispatcher<'static, 'static>) {
        dispatcher.dispatch(world);
        world.maintain();
    }
}

mod game_objects_vec_bench {
    use criterion::black_box;

    use crate::{increment_system, shuffle_system, GameObject, OtherData, Position};

    pub fn setup(n: usize) -> Vec<GameObject> {
        let mut game_objects = Vec::with_capacity(n);
        for i in 0..n {
            game_objects.push(GameObject {
                id: i as u64,
                position: if i % 4 != 0 {
                    Some(Position {
                        x: black_box(i as f64 * 0.1),
                        y: black_box(i as f64 * 0.1),
                        z: black_box(i as f64 * 0.1),
                    })
                } else {
                    None
                },
                other_data: if i % 3 == 0 {
                    Some(OtherData::default())
                } else {
                    None
                },
            });
        }
        game_objects
    }

    pub fn benchmark(game_objects: &mut [GameObject]) {
        for game_object in game_objects.iter_mut() {
            if let Some(pos) = &mut game_object.position {
                shuffle_system(pos);
            }
        }
        for game_object in game_objects.iter_mut() {
            if let Some(pos) = &mut game_object.position {
                increment_system(pos);
            }
        }
        for game_object in game_objects.iter_mut() {
            if let Some(pos) = &mut game_object.position {
                shuffle_system(pos);
            }
        }
    }
}

mod game_objects_hash_bench {
    use std::collections::HashMap;

    use criterion::black_box;

    use crate::{increment_system, shuffle_system, GameObject, OtherData, Position};

    pub fn setup(n: usize) -> HashMap<u64, GameObject> {
        let mut game_objects = HashMap::with_capacity(n);
        for i in 0..n {
            game_objects.insert(
                i as u64,
                GameObject {
                    id: i as u64,
                    position: if i % 4 != 0 {
                        Some(Position {
                            x: black_box(i as f64 * 0.1),
                            y: black_box(i as f64 * 0.1),
                            z: black_box(i as f64 * 0.1),
                        })
                    } else {
                        None
                    },
                    other_data: if i % 3 == 0 {
                        Some(OtherData::default())
                    } else {
                        None
                    },
                },
            );
        }
        game_objects
    }

    pub fn benchmark(game_objects: &mut HashMap<u64, GameObject>) {
        for game_object in game_objects.values_mut() {
            if let Some(pos) = &mut game_object.position {
                shuffle_system(pos);
            }
        }
        for game_object in game_objects.values_mut() {
            if let Some(pos) = &mut game_object.position {
                increment_system(pos);
            }
        }
        for game_object in game_objects.values_mut() {
            if let Some(pos) = &mut game_object.position {
                shuffle_system(pos);
            }
        }
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
