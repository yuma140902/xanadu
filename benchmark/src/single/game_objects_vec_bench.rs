use super::{increment_system, shuffle_system, GameObject, OtherData, Position};
use crate::black_box;

pub fn setup(n: usize) -> Vec<GameObject> {
    let mut game_objects = Vec::with_capacity(n);
    for i in 0..n {
        game_objects.push(GameObject {
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
        });
    }
    game_objects
}

pub fn benchmark(game_objects: &mut [GameObject]) {
    for game_object in game_objects.iter_mut() {
        if let Some(pos) = &mut game_object.position {
            shuffle_system(pos);
        }
    }
    for game_object in game_objects.iter_mut() {
        if let Some(pos) = &mut game_object.position {
            increment_system(pos);
        }
    }
    for game_object in game_objects.iter_mut() {
        if let Some(pos) = &mut game_object.position {
            shuffle_system(pos);
        }
    }
}
