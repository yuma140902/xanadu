use xanadu::ecs::dyn_pool::{Mut, World};

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

#[allow(clippy::missing_const_for_fn)]
fn main() {
    let mut world = World::builder()
        .register_component::<Position>()
        .register_component::<Velocity>()
        .build();

    const NUM_ENTITIES: usize = 1_000_000;
    for i in 0..NUM_ENTITIES {
        let entity = world.new_entity();
        world.attach_component(
            entity,
            Position {
                x: i as f64 * 0.1,
                y: i as f64 * 0.1,
                z: i as f64 * 0.1,
            },
        );
        world.attach_component(entity, Velocity::default());
    }

    let timer = std::time::Instant::now();
    world.execute::<'_, Position, _>(&print_system);
    world.execute::<'_, Mut<Position>, _>(&increment_system);
    world.execute::<'_, Position, _>(&print_system);
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
