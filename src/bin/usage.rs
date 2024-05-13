use xanadu::ecs::{SingleComponentExclusiveIter, SingleComponentExclusiveIterMut, World};

#[derive(Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

fn main() {
    let mut world = World::builder().register_component::<Position>().build();
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

    world.execute(print_system);
    world.execute(shuffle_system);
    world.execute(increment_system);
    world.execute(shuffle_system);
    println!("Shuffled and incremented");
    world.execute(print_system);
}

fn print_system(iter: SingleComponentExclusiveIter<'_, Position>) {
    for pos in iter {
        println!("Pos: [{}, {}, {}]", pos.x, pos.y, pos.z);
    }
}

fn shuffle_system(iter: SingleComponentExclusiveIterMut<'_, Position>) {
    for pos in iter {
        let tmp = pos.x;
        pos.x = pos.y;
        pos.y = pos.z;
        pos.z = tmp;
    }
}

fn increment_system(iter: SingleComponentExclusiveIterMut<'_, Position>) {
    for pos in iter {
        pos.x += 1.0;
        pos.y += 2.0;
        pos.z += 3.0;
    }
}
