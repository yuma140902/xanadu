/// Represents a component that can be attached to an entity.
pub trait Component: 'static {}

impl<T> Component for T where T: 'static {}
