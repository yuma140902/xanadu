use std::collections::HashMap;

use criterion::black_box;

use crate::{increment_system, shuffle_system, GameObject, OtherData, Position};

pub fn setup(n: usize) -> HashMap<u64, GameObject> {
    let mut game_objects = HashMap::with_capacity(n);
    for i in 0..n {
        game_objects.insert(
            i as u64,
            GameObject {
                id: i as u64,
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

pub fn benchmark(game_objects: &mut HashMap<u64, GameObject>) {
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
