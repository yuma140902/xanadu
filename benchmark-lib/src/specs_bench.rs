use specs::prelude::*;

use crate::{black_box, increment_system, shuffle_system, Id, OtherData, Position};

impl Component for Id {
    type Storage = VecStorage<Self>;
}

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
    world.register::<Id>();
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
        entity_builder = entity_builder.with(Id(i));
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

#[cfg(test)]
mod test {

    use super::*;
    use crate::{game_objects_vec_bench, GameObject};

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
    struct Entities(pub Vec<(Id, Position)>);

    struct EntitiesSystem;

    impl<'a> System<'a> for EntitiesSystem {
        type SystemData = (
            ReadStorage<'a, Id>,
            ReadStorage<'a, Position>,
            Write<'a, Entities>,
        );

        fn run(&mut self, (ids, positions, mut entities): Self::SystemData) {
            for (id, pos) in (&ids, &positions).join() {
                entities.0.push((id.clone(), pos.clone()));
            }
        }
    }

    fn assert_same(world: &mut World, vec: &[GameObject]) {
        world.insert(Entities(Vec::new()));

        let mut dispatcher = DispatcherBuilder::new()
            .with(EntitiesSystem, "entities_system", &[])
            .build();
        dispatcher.setup(world);
        dispatcher.dispatch(world);

        let entities = world.read_resource::<Entities>();
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
