use super::{FromWorld, World};

mod private {
    pub trait Sealed<'world, T> {}
}

/// Represents a system that can be executed on a world.
///
/// This trait is not meant to be implemented by the user. See Implementors section for types that
/// can be used as systems.
pub trait System<'world, T>: private::Sealed<'world, T> {
    fn execute(&self, world: &'world mut World);
}

impl<'world, T, F> private::Sealed<'world, T> for F
where
    T: FromWorld<'world>,
    F: Fn(T),
{
}

impl<'world, T, F> System<'world, T> for F
where
    T: FromWorld<'world>,
    F: Fn(T),
{
    fn execute(&self, world: &'world mut World) {
        self(T::from_world(world));
    }
}
