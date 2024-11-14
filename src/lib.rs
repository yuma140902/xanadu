#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]

//! # Xanadu
//!
//! Xanadu is a toy ECS library which works on Windows, Linux, macOS and WebAssembly.
//!
//! ## Example
//!
//! ```rust
//! use xanadu::ecs::{
//!     PairComponentsRefIter, PairComponentsRefIterMut, SingleComponentExclusiveIter,
//!     SingleComponentExclusiveIterMut, World,
//! };
//!
//! #[derive(Debug)]
//! pub struct Position {
//!     pub x: f64,
//!     pub y: f64,
//!     pub z: f64,
//! }
//!
//! #[derive(Debug)]
//! pub struct Velocity {
//!     pub x: f64,
//!     pub y: f64,
//!     pub z: f64,
//! }
//!
//! fn main() {
//!     let mut world = World::builder()
//!         .register_component::<Position>()
//!         .register_component::<Velocity>()
//!         .build();
//!     for i in 0..5 {
//!         let entity = world.new_entity();
//!         world.attach_component(
//!             entity,
//!             Position {
//!                 x: i as f64,
//!                 y: i as f64,
//!                 z: i as f64,
//!             },
//!         );
//!     }
//!     for i in 0..3 {
//!         let entity = world.new_entity();
//!         world.attach_component(
//!             entity,
//!             Position {
//!                 x: i as f64,
//!                 y: i as f64,
//!                 z: i as f64,
//!             },
//!         );
//!         world.attach_component(
//!             entity,
//!             Velocity {
//!                 x: i as f64 * 0.1,
//!                 y: i as f64 * 0.1,
//!                 z: i as f64 * 0.1,
//!             },
//!         );
//!     }
//!
//!     world.execute(print_system);
//!     world.execute(shuffle_system);
//!     world.execute(increment_system);
//!     world.execute(shuffle_system);
//!     println!("Shuffled and incremented");
//!     world.execute(print_system);
//!     println!("===================");
//!     world.execute(print2_system);
//!     println!("Applying velocity");
//!     world.execute(apply_velocity_system);
//!     world.execute(print2_system);
//! }
//!
//! fn print_system(iter: SingleComponentExclusiveIter<'_, Position>) {
//!     for pos in iter {
//!         println!("Pos: [{}, {}, {}]", pos.x, pos.y, pos.z);
//!     }
//! }
//!
//! fn shuffle_system(iter: SingleComponentExclusiveIterMut<'_, Position>) {
//!     for pos in iter {
//!         let tmp = pos.x;
//!         pos.x = pos.y;
//!         pos.y = pos.z;
//!         pos.z = tmp;
//!     }
//! }
//!
//! fn increment_system(iter: SingleComponentExclusiveIterMut<'_, Position>) {
//!     for pos in iter {
//!         pos.x += 1.0;
//!         pos.y += 2.0;
//!         pos.z += 3.0;
//!     }
//! }
//!
//! fn print2_system(iter: PairComponentsRefIter<'_, Position, Velocity>) {
//!     for (pos, vel) in iter {
//!         println!(
//!             "Pos: [{}, {}, {}] Vel: [{}, {}, {}]",
//!             pos.x, pos.y, pos.z, vel.x, vel.y, vel.z
//!         );
//!     }
//! }
//!
//! fn apply_velocity_system(iter: PairComponentsRefIterMut<'_, Position, Velocity>) {
//!     for (mut pos, vel) in iter {
//!         pos.x += vel.x;
//!         pos.y += vel.y;
//!         pos.z += vel.z;
//!     }
//! }
//! ```

/// Collections to be used in ECS, but can be used independently.
pub mod collections;

/// ECS module; main module of this library.
///
/// # ECS
///
/// ECS stands for Entity-Component-System. It is a design pattern used in game development. It is
/// known for its performance when dealing with large number of entities.
///
/// # Entities
///
/// Entities are unique identifiers that have no intrinsic properties on their own. They are just
/// identifiers used to keep track of components attached to them. In Xanadu, entities are just
/// [`GenerationalId`](collections::GenerationalId)s.
///
/// # Components
///
/// Components are data structures that can be attached to entities. In Xanadu, components are
/// types which implement [`Component`](ecs::Component) trait.
///
/// # Systems
///
/// Systems are functions that operate on components. In Xanadu, systems are types which implement
/// [`System`](ecs::System) trait. They are usually functions that take a reference to a component
/// and return nothing.
pub mod ecs;
