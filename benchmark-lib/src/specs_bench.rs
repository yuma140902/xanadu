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
