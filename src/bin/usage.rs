use xanadu::ecs::{
    PairComponentIter, PairComponentIterMut, SingleComponentIter, SingleComponentIterMut, World,
};

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

fn main() {
    let mut world = World::builder()
        .register_component::<Position>()
        .register_component::<Velocity>()
        .build();
    for i in 0..5 {
        let entity = world.new_entity();
        world.attach_component(
            entity,
            Position {
                x: i as f64,
                y: i as f64,
                z: i as f64,
            },
        );
    }
    for i in 0..3 {
        let entity = world.new_entity();
        world.attach_component(
            entity,
            Position {
                x: i as f64,
                y: i as f64,
                z: i as f64,
            },
        );
        world.attach_component(
            entity,
            Velocity {
                x: i as f64,
                y: i as f64,
                z: i as f64,
            },
        );
    }

    world.execute(&print_system);
    world.execute(&print2_system);
    world.execute(&shuffle_system);
    world.execute(&increment_system);
    world.execute(&shuffle_system);
    println!("Shuffled and incremented");
    world.execute(&print_system);
    println!("====================");
    world.execute(&print2_system);
    world.execute(&apply_force_system);
    println!("====================");
    world.execute(&print2_system);
}

fn print_system(iter: SingleComponentIter<'_, Position>) {
    for pos in iter {
        println!("Pos: [{}, {}, {}]", pos.x, pos.y, pos.z);
    }
}

fn shuffle_system(iter: SingleComponentIterMut<'_, Position>) {
    for pos in iter {
        let tmp = pos.x;
        pos.x = pos.y;
        pos.y = pos.z;
        pos.z = tmp;
    }
}

fn increment_system(iter: SingleComponentIterMut<'_, Position>) {
    for pos in iter {
        pos.x += 1.0;
        pos.y += 2.0;
        pos.z += 3.0;
    }
}

fn print2_system(iter: PairComponentIter<'_, Position, Velocity>) {
    for (pos, vel) in iter {
        println!(
            "Pos: [{}, {}, {}], Vel: [{}, {}, {}]",
            pos.x, pos.y, pos.z, vel.x, vel.y, vel.z
        );
    }
}

fn apply_force_system(iter: PairComponentIterMut<'_, Position, Velocity>) {
    for (pos, vel) in iter {
        pos.x += vel.x;
        pos.y += vel.y;
        pos.z += vel.z;
    }
}
