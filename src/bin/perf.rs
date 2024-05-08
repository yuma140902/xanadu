use xanadu::ecs::dyn_pool::World;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct GameObject {
    pub id: u64,
    pub position: Position,
    pub other_data: [u8; 128],
}

#[allow(clippy::missing_const_for_fn)]
fn main() {
    let mut world = World::builder().register_component::<Position>().build();

    const NUM_ENTITIES: usize = 1_000_000;
    let mut entities = Vec::with_capacity(NUM_ENTITIES);
    for i in 0..NUM_ENTITIES {
        let entity = world.new_entity();
        entities.push(entity);
        world.attach_component(
            entity,
            Position {
                x: i as f64 * 0.1,
                y: i as f64 * 0.1,
                z: i as f64 * 0.1,
            },
        );
    }
    eprintln!("Number of entities: {}", entities.len());

    let mut game_objects = Vec::with_capacity(NUM_ENTITIES);
    for i in 0..NUM_ENTITIES {
        game_objects.push(GameObject {
            id: i as u64,
            position: Position {
                x: -(i as f64 * 0.1),
                y: -(i as f64 * 0.1),
                z: -(i as f64 * 0.1),
            },
            other_data: [0; 128],
        });
    }
    eprintln!("Number of game objects: {}", game_objects.len());

    let timer = std::time::Instant::now();
    world.execute::<'_, Position, _>(&print_system);
    eprintln!("ECS: {:?}", timer.elapsed());

    let timer = std::time::Instant::now();
    for game_object in game_objects.iter() {
        print_system(&game_object.position);
    }
    eprintln!("GameObject: {:?}", timer.elapsed());
}

fn print_system(pos: &Position) {
    println!("Pos: [{}, {}, {}]", pos.x, pos.y, pos.z);
}
