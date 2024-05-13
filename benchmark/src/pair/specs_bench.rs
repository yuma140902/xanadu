use specs::prelude::*;

use super::{apply_velocity_system, decay_velocity_system, Id, Position, Velocity};
use crate::black_box;

impl Component for Id {
    type Storage = VecStorage<Self>;
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

struct DecayVelocitySystem;

impl<'a> System<'a> for DecayVelocitySystem {
    type SystemData = WriteStorage<'a, Velocity>;

    fn run(&mut self, mut positions: Self::SystemData) {
        for pos in (&mut positions).join() {
            decay_velocity_system(pos);
        }
    }
}

struct ApplyVelocitySystem;

impl<'a> System<'a> for ApplyVelocitySystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);
    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (pos, vel) in (&mut positions, &velocities).join() {
            apply_velocity_system(pos, vel)
        }
    }
}

pub fn setup(n: usize) -> (World, Dispatcher<'static, 'static>) {
    let mut world = World::new();
    world.register::<Id>();
    world.register::<Position>();
    world.register::<Velocity>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(DecayVelocitySystem, "decay1", &[])
        .with(ApplyVelocitySystem, "apply", &["decay1"])
        .with(DecayVelocitySystem, "decay2", &["apply"])
        .build();
    dispatcher.setup(&mut world);

    for i in 0..n {
        let mut entity_builder = world.create_entity();
        entity_builder = entity_builder.with(Id(i));
        if (i / 10) % 3 != 0 {
            entity_builder = entity_builder.with(Position {
                x: black_box(i as f64) + 3.0,
                y: black_box(i as f64) + 2.0,
                z: black_box(i as f64) + 1.0,
            });
        }
        if (i / 10) % 4 != 0 {
            entity_builder = entity_builder.with(Velocity {
                x: black_box(i as f64).mul_add(5.0, 3.0),
                y: black_box(i as f64).mul_add(5.0, 2.0),
                z: black_box(i as f64).mul_add(5.0, 1.0),
            });
        }
        entity_builder.build();
    }

    (world, dispatcher)
}

pub fn benchmark(world: &mut World, dispatcher: &mut Dispatcher<'static, 'static>) {
    dispatcher.dispatch(world);
    world.maintain();
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::pair::{game_objects_vec_bench, GameObject};

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

        let (mut world, mut dispatcher) = setup(30);
        benchmark(&mut world, &mut dispatcher);

        assert_same(&mut world, &vec);
    }

    #[derive(Default)]
    struct EntitiesWithPos(pub Vec<(Id, Position)>);

    struct EntitiesWithPosSystem;

    impl<'a> System<'a> for EntitiesWithPosSystem {
        type SystemData = (
            ReadStorage<'a, Id>,
            ReadStorage<'a, Position>,
            Write<'a, EntitiesWithPos>,
        );

        fn run(&mut self, (ids, positions, mut entities): Self::SystemData) {
            for (id, pos) in (&ids, &positions).join() {
                entities.0.push((id.clone(), pos.clone()));
            }
        }
    }

    #[derive(Default)]
    struct EntitiesWithVel(pub Vec<(Id, Velocity)>);

    struct EntitiesWithVelSystem;

    impl<'a> System<'a> for EntitiesWithVelSystem {
        type SystemData = (
            ReadStorage<'a, Id>,
            ReadStorage<'a, Velocity>,
            Write<'a, EntitiesWithVel>,
        );
        fn run(&mut self, (ids, velocities, mut entities): Self::SystemData) {
            for (id, vel) in (&ids, &velocities).join() {
                entities.0.push((id.clone(), vel.clone()));
            }
        }
    }

    fn assert_same(world: &mut World, vec: &[GameObject]) {
        world.insert(EntitiesWithPos(Vec::new()));
        world.insert(EntitiesWithVel(Vec::new()));

        let mut dispatcher = DispatcherBuilder::new()
            .with(EntitiesWithPosSystem, "pos", &[])
            .with(EntitiesWithVelSystem, "vel", &[])
            .build();
        dispatcher.setup(world);
        dispatcher.dispatch(world);

        let entities = world.read_resource::<EntitiesWithPos>();
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

        let entities = world.read_resource::<EntitiesWithVel>();
        for (id, vel) in entities.0.iter() {
            let game_object = vec.iter().find(|v| v.id == id.0).unwrap();
            let game_vel = game_object.velocity.as_ref().unwrap();
            assert_eq!(vel.x, game_vel.x);
            assert_eq!(vel.y, game_vel.y);
            assert_eq!(vel.z, game_vel.z);
        }
        assert_eq!(
            entities.0.len(),
            vec.iter().filter(|o| o.velocity.is_some()).count()
        );
    }
}
