use std::collections::HashMap;

use super::{apply_velocity_system, decay_velocity_system, GameObject, Position, Velocity};
use crate::black_box;

pub fn setup(n: usize) -> HashMap<usize, GameObject> {
    let mut game_objects = HashMap::with_capacity(n);
    for i in 0..n {
        game_objects.insert(
            i,
            GameObject {
                id: i,
                position: if (i / 10) % 3 != 0 {
                    Some(Position {
                        x: black_box(i as f64) + 3.0,
                        y: black_box(i as f64) + 2.0,
                        z: black_box(i as f64) + 1.0,
                    })
                } else {
                    None
                },
                velocity: if (i / 10) % 4 != 0 {
                    Some(Velocity {
                        x: black_box(i as f64) * 5.0 + 3.0,
                        y: black_box(i as f64) * 5.0 + 2.0,
                        z: black_box(i as f64) * 5.0 + 1.0,
                    })
                } else {
                    None
                },
            },
        );
    }
    game_objects
}

pub fn benchmark(game_objects: &mut HashMap<usize, GameObject>) {
    for game_object in game_objects.values_mut() {
        if let Some(vel) = &mut game_object.velocity {
            decay_velocity_system(vel);
        }
    }
    for game_object in game_objects.values_mut() {
        if let (Some(pos), Some(vel)) = (&mut game_object.position, &game_object.velocity) {
            apply_velocity_system(pos, vel);
        }
    }
    for game_object in game_objects.values_mut() {
        if let Some(vel) = &mut game_object.velocity {
            decay_velocity_system(vel);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pair::game_objects_vec_bench;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;
    #[cfg(all(target_arch = "wasm32", feature = "test_in_browser"))]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn setup_test() {
        let vec = game_objects_vec_bench::setup(30);
        let hash = setup(30);
        assert_same(&hash, &vec);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn systems_test() {
        let mut vec = game_objects_vec_bench::setup(30);
        game_objects_vec_bench::benchmark(&mut vec);

        let mut hash = setup(30);
        benchmark(&mut hash);

        assert_same(&hash, &vec);
    }

    fn assert_same(hash: &HashMap<usize, GameObject>, vec: &[GameObject]) {
        for (hash_key, hash_value) in hash.iter() {
            let vec_value = vec.iter().find(|v| v.id == *hash_key).unwrap();
            let pos = &hash_value.position;
            let vel = &hash_value.velocity;
            assert_eq!(pos.is_none(), vec_value.position.is_none());
            if let (Some(pos1), Some(pos2)) = (pos, &vec_value.position) {
                assert_eq!(pos1.x, pos2.x);
                assert_eq!(pos1.y, pos2.y);
                assert_eq!(pos1.z, pos2.z);
            }
            assert_eq!(vel.is_none(), vec_value.velocity.is_none());
            if let (Some(vel1), Some(vel2)) = (vel, &vec_value.velocity) {
                assert_eq!(vel1.x, vel2.x);
                assert_eq!(vel1.y, vel2.y);
                assert_eq!(vel1.z, vel2.z);
            }
        }

        assert_eq!(hash.len(), vec.len());
    }
}
