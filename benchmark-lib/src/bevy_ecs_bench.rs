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
