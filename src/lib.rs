//! # Xanadu
//!
//! Xanadu is a toy ECS library which works on Windows, Linux, macOS and WebAssembly.
//!
//! ## Example
//!
//! ```rust
//! use xanadu::ecs::{Mut, World};
//!
//! #[repr(C)]
//! #[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
//! pub struct Position {
//!     pub x: f64,
//!     pub y: f64,
//!     pub z: f64,
//! }
//!
//! fn main() {
//!     let mut world = World::builder().register_component::<Position>().build();
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
//!
//!     world.execute::<'_, Position, _>(&print_system);
//!     world.execute::<'_, Mut<Position>, _>(&shuffle_system);
//!     world.execute::<'_, Mut<Position>, _>(&increment_system);
//!     world.execute::<'_, Mut<Position>, _>(&shuffle_system);
//!     println!("Shuffled and incremented");
//!     world.execute::<'_, Position, _>(&print_system);
//! }
//!
//! fn print_system(pos: &Position) {
//!     println!("Pos: [{}, {}, {}]", pos.x, pos.y, pos.z);
//! }
//!
//! fn shuffle_system(pos: &mut Position) {
//!     let tmp = pos.x;
//!     pos.x = pos.y;
//!     pos.y = pos.z;
//!     pos.z = tmp;
//! }
//!
//! fn increment_system(pos: &mut Position) {
//!     pos.x += 1.0;
//!     pos.y += 2.0;
//!     pos.z += 3.0;
//! }
//! ```

/// Collections to be used in ECS, but can be used independently.
pub mod collections;

/// ECS module; main module of this library.
pub mod ecs;
