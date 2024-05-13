use xanadu::ecs::{PairComponentsRefIterMut, SingleComponentExclusiveIterMut, World};

use super::{apply_velocity_system_refcell, decay_velocity_system, Id, Position, Velocity};
use crate::black_box;

pub fn setup(n: usize) -> World {
    let mut world = World::builder()
        .register_component::<Id>()
        .register_component::<Position>()
        .register_component::<Velocity>()
        .build();

    for i in 0..n {
        let entity = world.new_entity();
        world.attach_component(entity, Id(i));
        if (i / 10) % 3 != 0 {
            world.attach_component(
                entity,
                Position {
                    x: black_box(i as f64) + 3.0,
                    y: black_box(i as f64) + 2.0,
                    z: black_box(i as f64) + 1.0,
                },
            );
        }
        if (i / 10) % 4 != 0 {
            world.attach_component(
                entity,
                Velocity {
                    x: black_box(i as f64) * 5.0 + 3.0,
                    y: black_box(i as f64) * 5.0 + 2.0,
                    z: black_box(i as f64) * 5.0 + 1.0,
                },
            );
        }
    }

    world
}

pub fn benchmark(world: &mut World) {
    world.execute(decay_velocity_system_xanadu);
    world.execute(apply_velocity_system_xanadu);
    world.execute(decay_velocity_system_xanadu);
}

fn apply_velocity_system_xanadu(iter: PairComponentsRefIterMut<'_, Position, Velocity>) {
    for (mut pos, mut vel) in iter {
        apply_velocity_system_refcell(&mut pos, &mut vel)
    }
}

fn decay_velocity_system_xanadu(iter: SingleComponentExclusiveIterMut<'_, Velocity>) {
    for pos in iter {
        decay_velocity_system(pos);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pair::{game_objects_vec_bench, GameObject};
    use xanadu::ecs::SingleComponentExclusiveIter;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;
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

    fn assert_same(game_objects: &[GameObject], world: &mut World) {
        let mut positions = Vec::new();
        world.execute(|iter: SingleComponentExclusiveIter<'_, Position>| {
            for pos in iter {
                positions.push(pos.clone());
            }
        });

        let mut velocities = Vec::new();
        world.execute(|iter: SingleComponentExclusiveIter<'_, Velocity>| {
            for vel in iter {
                velocities.push(vel.clone());
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

        for (vel1, vel2) in velocities
            .iter()
            .zip(game_objects.iter().filter_map(|x| x.velocity.as_ref()))
        {
            assert_eq!(vel1.x, vel2.x);
            assert_eq!(vel1.y, vel2.y);
            assert_eq!(vel1.z, vel2.z);
        }
        assert_eq!(
            velocities.len(),
            game_objects
                .iter()
                .filter_map(|x| x.velocity.as_ref())
                .count()
        );
    }
}
