use super::{Component, Query, World};

pub trait System<'world, Q, T>
where
    Q: Query<'world, T>,
{
    fn execute(&self, world: &'world World);
}

impl<'world, Q, C, F> System<'world, Q, &'world C> for F
where
    C: Component,
    Q: Query<'world, &'world C>,
    F: Fn(&'world C),
{
    fn execute(&self, world: &'world World) {
        for component in Q::get_iterator(world) {
            self(component);
        }
    }
}
