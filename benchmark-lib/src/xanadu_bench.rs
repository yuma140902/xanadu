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
