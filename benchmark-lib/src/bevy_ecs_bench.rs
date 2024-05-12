use bevy_ecs::prelude::*;

use crate::{black_box, increment_system, shuffle_system, Id, OtherData, Position};

pub fn setup(n: usize) -> (World, Schedule) {
    let mut world = World::new();
    for i in 0..n {
        if i % 4 != 0 && i % 3 == 0 {
            world.spawn((
                Id(i),
                Position {
                    x: black_box(i as f64 * 0.1),
                    y: black_box(i as f64 * 0.1),
                    z: black_box(i as f64 * 0.1),
                },
                OtherData::default(),
            ));
        } else if i % 4 != 0 {
            world.spawn((
                Id(i),
                Position {
                    x: black_box(i as f64 * 0.1),
                    y: black_box(i as f64 * 0.1),
                    z: black_box(i as f64 * 0.1),
                },
            ));
        } else if i % 3 == 0 {
            world.spawn((Id(i), OtherData::default()));
        } else {
            world.spawn(());
        }
    }

    let mut schedule = Schedule::default();
    schedule.add_systems(shuffle_system_bevy1);
    schedule.add_systems(shuffle_system_bevy2);
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

fn shuffle_system_bevy1(mut query: Query<(&mut Position,)>) {
    for (mut pos,) in query.iter_mut() {
        shuffle_system(&mut pos);
    }
}

fn shuffle_system_bevy2(mut query: Query<(&mut Position,)>) {
    for (mut pos,) in query.iter_mut() {
        shuffle_system(&mut pos);
    }
}

fn increment_system_bevy(mut query: Query<(&mut Position,)>) {
    for (mut pos,) in query.iter_mut() {
        increment_system(&mut pos);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{game_objects_vec_bench, GameObject, Id};

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

    struct Entities(pub Vec<(Id, Position)>);

    impl Resource for Entities {}
    impl FromWorld for Entities {
        fn from_world(_world: &mut World) -> Self {
            Entities(Vec::new())
        }
    }

    fn entities_system(mut query: Query<(&Id, &Position)>, mut entities: ResMut<Entities>) {
        for (id, pos) in query.iter_mut() {
            entities.0.push((id.clone(), pos.clone()));
        }
    }

    fn assert_same(world: &mut World, vec: &[GameObject]) {
        world.init_resource::<Entities>();
        let mut schedule = Schedule::default();
        schedule.add_systems(entities_system);

        schedule.run(world);
        let entities = world.resource::<Entities>();

        for (id, pos) in entities.0.iter() {
            let game_object = vec.iter().find(|v| v.id == id.0).unwrap();
            let game_pos = game_object.position.as_ref().unwrap();
            assert_eq!(pos.x, game_pos.x);
            assert_eq!(pos.y, game_pos.y);
            assert_eq!(pos.z, game_pos.z);
        }
        assert_eq!(
            entities.0.len(),
            vec.iter().filter(|o| o.position.is_some()).count()
        );
    }
}
