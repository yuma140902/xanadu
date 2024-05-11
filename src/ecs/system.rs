use super::{Component, Query, World};

pub trait System<'world, Q, T>
where
    Q: Query<'world, T>,
{
    fn execute(&self, world: &'world mut World);
}

impl<'world, Q, C, F> System<'world, Q, &'world C> for F
where
    C: Component,
    Q: Query<'world, &'world C>,
    F: Fn(&'world C),
{
    fn execute(&self, world: &'world mut World) {
        for component in Q::get_iterator(world) {
            self(component);
        }
    }
}

impl<'world, Q, C, F> System<'world, Q, &'world mut C> for F
where
    C: Component,
    Q: Query<'world, &'world mut C>,
    F: Fn(&'world mut C),
{
    fn execute(&self, world: &'world mut World) {
        for component in Q::get_iterator(world) {
            self(component);
        }
    }
}
