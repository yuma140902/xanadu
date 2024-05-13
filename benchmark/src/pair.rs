#[derive(Debug, Clone, PartialEq, Eq, bevy_ecs::prelude::Component)]
pub struct Id(usize);

#[derive(Debug, Clone, PartialEq, bevy_ecs::prelude::Component)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, PartialEq, bevy_ecs::prelude::Component)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct GameObject {
    pub id: usize,
    pub position: Position,
    pub velocity: Velocity,
}
