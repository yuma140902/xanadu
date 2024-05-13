use super::{apply_velocity_system, decay_velocity_system, GameObject, Position, Velocity};
use crate::black_box;

pub fn setup(n: usize) -> Vec<GameObject> {
    let mut game_objects = Vec::with_capacity(n);
    for i in 0..n {
        game_objects.push(GameObject {
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
        });
    }
    game_objects
}

pub fn benchmark(game_objects: &mut [GameObject]) {
    for game_object in game_objects.iter_mut() {
        if let Some(vel) = &mut game_object.velocity {
            decay_velocity_system(vel);
        }
    }
    for game_object in game_objects.iter_mut() {
        if let (Some(pos), Some(vel)) = (&mut game_object.position, &game_object.velocity) {
            apply_velocity_system(pos, vel);
        }
    }
    for game_object in game_objects.iter_mut() {
        if let Some(vel) = &mut game_object.velocity {
            decay_velocity_system(vel);
        }
    }
}
