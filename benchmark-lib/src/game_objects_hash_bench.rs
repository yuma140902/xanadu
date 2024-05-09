use std::collections::HashMap;

use criterion::black_box;

use crate::{increment_system, shuffle_system, GameObject, OtherData, Position};

pub fn setup(n: usize) -> HashMap<usize, GameObject> {
    let mut game_objects = HashMap::with_capacity(n);
    for i in 0..n {
        game_objects.insert(
            i,
            GameObject {
                id: i,
                position: if i % 4 != 0 {
                    Some(Position {
                        x: black_box(i as f64 * 0.1),
                        y: black_box(i as f64 * 0.1),
                        z: black_box(i as f64 * 0.1),
                    })
                } else {
                    None
                },
                other_data: if i % 3 == 0 {
                    Some(OtherData::default())
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
        if let Some(pos) = &mut game_object.position {
            shuffle_system(pos);
        }
    }
    for game_object in game_objects.values_mut() {
        if let Some(pos) = &mut game_object.position {
            increment_system(pos);
        }
    }
    for game_object in game_objects.values_mut() {
        if let Some(pos) = &mut game_object.position {
            shuffle_system(pos);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game_objects_vec_bench;

    #[test]
    fn setup_test() {
        let vec = game_objects_vec_bench::setup(30);
        let hash = setup(30);
        assert_same(&hash, &vec);
    }

    #[test]
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
            let pos = hash_value.position;
            assert_eq!(pos.is_none(), vec_value.position.is_none());
            if let (Some(pos1), Some(pos2)) = (pos, vec_value.position) {
                assert_eq!(pos1.x, pos2.x);
                assert_eq!(pos1.y, pos2.y);
                assert_eq!(pos1.z, pos2.z);
            }
        }

        assert_eq!(hash.len(), vec.len());
    }
}
