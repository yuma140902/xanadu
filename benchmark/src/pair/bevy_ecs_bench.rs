use bevy_ecs::prelude::*;

use super::{apply_velocity_system, decay_velocity_system, Id, Position, Velocity};
use crate::black_box;

pub fn setup(n: usize) -> (World, Schedule) {
    let mut world = World::new();
    for i in 0..n {
        if (i / 10) % 3 != 0 && (i / 10) % 4 != 0 {
            world.spawn((
                Id(i),
                Position {
                    x: black_box(i as f64) + 3.0,
                    y: black_box(i as f64) + 2.0,
                    z: black_box(i as f64) + 1.0,
                },
                Velocity {
                    x: black_box(i as f64).mul_add(5.0, 3.0),
                    y: black_box(i as f64).mul_add(5.0, 2.0),
                    z: black_box(i as f64).mul_add(5.0, 1.0),
                },
            ));
        } else if (i / 10) % 3 != 0 {
            world.spawn((
                Id(i),
                Position {
                    x: black_box(i as f64) + 3.0,
                    y: black_box(i as f64) + 2.0,
                    z: black_box(i as f64) + 1.0,
                },
            ));
        } else if (i / 10) % 4 != 0 {
            world.spawn((
                Id(i),
                Velocity {
                    x: black_box(i as f64).mul_add(5.0, 3.0),
                    y: black_box(i as f64).mul_add(5.0, 2.0),
                    z: black_box(i as f64).mul_add(5.0, 1.0),
                },
            ));
        } else {
            world.spawn((Id(i),));
        }
    }

    let mut schedule = Schedule::default();
    schedule.add_systems(decay_velocity_system_bevy1);
    schedule.add_systems(decay_velocity_system_bevy2);
    schedule.add_systems(
        apply_velocity_system_bevy
            .after(decay_velocity_system_bevy1)
            .before(decay_velocity_system_bevy2),
    );

    (world, schedule)
}

pub fn benchmark(world: &mut World, schedule: &mut Schedule) {
    schedule.run(world);
}

fn decay_velocity_system_bevy1(mut query: Query<'_, '_, (&mut Velocity,)>) {
    for (mut vel,) in query.iter_mut() {
        decay_velocity_system(&mut vel);
    }
}

fn decay_velocity_system_bevy2(mut query: Query<'_, '_, (&mut Velocity,)>) {
    for (mut vel,) in query.iter_mut() {
        decay_velocity_system(&mut vel);
    }
}

fn apply_velocity_system_bevy(mut query: Query<'_, '_, (&mut Position, &Velocity)>) {
    for (mut pos, vel) in query.iter_mut() {
        apply_velocity_system(&mut pos, vel);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pair::{game_objects_vec_bench, GameObject, Id};

    #[test]
    fn setup_test() {
        let vec = game_objects_vec_bench::setup(30);
        let (mut world, _) = setup(30);

        assert_same(&mut world, &vec);
    }

    #[test]
    fn systems_test() {
        let mut vec = game_objects_vec_bench::setup(30);
        game_objects_vec_bench::benchmark(&mut vec);

        let (mut world, mut schedule) = setup(30);
        benchmark(&mut world, &mut schedule);

        assert_same(&mut world, &vec);
    }

    struct EntitiesWithPos(pub Vec<(Id, Position)>);
    struct EntitiesWithVel(pub Vec<(Id, Velocity)>);

    impl Resource for EntitiesWithPos {}
    impl FromWorld for EntitiesWithPos {
        fn from_world(_world: &mut World) -> Self {
            Self(Vec::new())
        }
    }

    impl Resource for EntitiesWithVel {}
    impl FromWorld for EntitiesWithVel {
        fn from_world(_world: &mut World) -> Self {
            Self(Vec::new())
        }
    }

    fn entities_with_pos_system(
        mut query: Query<'_, '_, (&Id, &Position)>,
        mut entities: ResMut<'_, EntitiesWithPos>,
    ) {
        for (id, pos) in query.iter_mut() {
            entities.0.push((id.clone(), pos.clone()));
        }
    }

    fn entities_with_vel_system(
        mut query: Query<'_, '_, (&Id, &Velocity)>,
        mut entities: ResMut<'_, EntitiesWithVel>,
    ) {
        for (id, vel) in query.iter_mut() {
            entities.0.push((id.clone(), vel.clone()));
        }
    }

    fn assert_same(world: &mut World, vec: &[GameObject]) {
        world.init_resource::<EntitiesWithPos>();
        world.init_resource::<EntitiesWithVel>();

        let mut schedule = Schedule::default();
        schedule.add_systems(entities_with_pos_system);
        schedule.add_systems(entities_with_vel_system);

        schedule.run(world);
        let entities_with_pos = world.resource::<EntitiesWithPos>();
        let entities_with_vel = world.resource::<EntitiesWithVel>();

        for (id, pos) in entities_with_pos.0.iter() {
            let game_object = vec.iter().find(|v| v.id == id.0).unwrap();
            let game_pos = game_object.position.as_ref().unwrap();
            assert_eq!(pos.x, game_pos.x);
            assert_eq!(pos.y, game_pos.y);
            assert_eq!(pos.z, game_pos.z);
        }
        assert_eq!(
            entities_with_pos.0.len(),
            vec.iter().filter(|o| o.position.is_some()).count()
        );

        for (id, vel) in entities_with_vel.0.iter() {
            let game_object = vec.iter().find(|v| v.id == id.0).unwrap();
            let game_vel = game_object.velocity.as_ref().unwrap();
            assert_eq!(vel.x, game_vel.x);
            assert_eq!(vel.y, game_vel.y);
            assert_eq!(vel.z, game_vel.z);
        }
        assert_eq!(
            entities_with_vel.0.len(),
            vec.iter().filter(|o| o.velocity.is_some()).count()
        );
    }
}
