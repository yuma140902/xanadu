use xanadu::ecs::{SingleComponentExclusiveIterMut, World};

use super::{increment_system, shuffle_system, Id, OtherData, Position};
use crate::black_box;

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
    world.execute(shuffle_system_xanadu);
    world.execute(increment_system_xanadu);
    world.execute(shuffle_system_xanadu);
}

fn shuffle_system_xanadu(iter: SingleComponentExclusiveIterMut<'_, Position>) {
    for pos in iter {
        shuffle_system(pos);
    }
}

fn increment_system_xanadu(iter: SingleComponentExclusiveIterMut<'_, Position>) {
    for pos in iter {
        increment_system(pos);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::single::game_objects_vec_bench;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;
    use xanadu::ecs::SingleComponentExclusiveIter;
    #[cfg(all(target_arch = "wasm32", feature = "test_in_browser"))]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn setup_test() {
        let game_objects = game_objects_vec_bench::setup(30);
        let mut world = setup(30);

        assert_same(&game_objects, &mut world);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn systems_test() {
        let mut game_objects = game_objects_vec_bench::setup(30);
        game_objects_vec_bench::benchmark(&mut game_objects);

        let mut world = setup(30);
        benchmark(&mut world);

        assert_same(&game_objects, &mut world);
    }

    fn assert_same(game_objects: &[crate::GameObject], world: &mut World) {
        let mut positions = Vec::new();
        world.execute(|iter: SingleComponentExclusiveIter<'_, Position>| {
            for pos in iter {
                positions.push(pos.clone());
            }
        });
        for (pos1, pos2) in positions
            .iter()
            .zip(game_objects.iter().filter_map(|x| x.position.as_ref()))
        {
            assert_eq!(pos1.x, pos2.x);
            assert_eq!(pos1.y, pos2.y);
            assert_eq!(pos1.z, pos2.z);
        }
        assert_eq!(
            positions.len(),
            game_objects
                .iter()
                .filter_map(|x| x.position.as_ref())
                .count()
        );
    }
}
