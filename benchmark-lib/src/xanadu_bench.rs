use criterion::black_box;
use xanadu::ecs::{Mut, World};

use crate::{increment_system, shuffle_system, Id, OtherData, Position};

pub fn setup(n: usize) -> World {
    let mut world = World::builder()
        .register_component::<Id>()
        .register_component::<Position>()
        .register_component::<OtherData>()
        .build();

    for i in 0..n {
        let entity = world.new_entity();
        world.attach_component(entity, Id(i));
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::game_objects_vec_bench;

    #[test]
    fn setup_test() {
        let game_objects = game_objects_vec_bench::setup(30);
        let world = setup(30);

        assert_same(&game_objects, &world);
    }

    #[test]
    fn systems_test() {
        let mut game_objects = game_objects_vec_bench::setup(30);
        game_objects_vec_bench::benchmark(&mut game_objects);

        let mut world = setup(30);
        benchmark(&mut world);

        assert_same(&game_objects, &world);
    }

    fn assert_same(game_objects: &[crate::GameObject], world: &World) {
        let pos_array = world.get_component_array::<Position>().unwrap();
        for (i, pos) in pos_array.data_iter().enumerate() {
            assert_eq!(pos.is_none(), game_objects[i].position.is_none());
            if let (Some(pos1), Some(pos2)) = (pos, game_objects[i].position) {
                assert_eq!(pos1.x, pos2.x);
                assert_eq!(pos1.y, pos2.y);
                assert_eq!(pos1.z, pos2.z);
            }
        }
        assert_eq!(pos_array.data_iter().len(), game_objects.len());
    }
}
