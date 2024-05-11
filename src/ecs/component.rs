/// Represents a component that can be attached to an entity.
pub trait Component: bytemuck::Pod {}

impl<T> Component for T where T: bytemuck::Pod {}
