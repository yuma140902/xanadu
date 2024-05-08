#[repr(C)]
#[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct GameObject {
    pub id: u64,
    pub position: Position,
    pub velocity: Velocity,
}

#[allow(clippy::missing_const_for_fn)]
fn main() {
    const NUM_ENTITIES: usize = 1_000_000;

    let mut game_objects = Vec::with_capacity(NUM_ENTITIES);
    for i in 0..NUM_ENTITIES {
        game_objects.push(GameObject {
            id: i as u64,
            position: Position {
                x: -(i as f64 * 0.1),
                y: -(i as f64 * 0.1),
                z: -(i as f64 * 0.1),
            },
            velocity: Velocity::default(),
        });
    }

    let timer = std::time::Instant::now();
    for game_object in game_objects.iter() {
        print_system(&game_object.position);
    }
    for game_object in game_objects.iter_mut() {
        increment_system(&mut game_object.position);
    }
    for game_object in game_objects.iter() {
        print_system(&game_object.position);
    }
    eprintln!("{:?}", timer.elapsed());
}

fn print_system(pos: &Position) {
    println!("Pos: [{}, {}, {}]", pos.x, pos.y, pos.z);
}

fn increment_system(pos: &mut Position) {
    pos.x += 1.0;
    pos.y += 2.0;
    pos.z += 3.0;
}
