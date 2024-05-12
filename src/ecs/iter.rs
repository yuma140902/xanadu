use super::{Component, World};

pub trait FromWorld<'world> {
    fn from_world(world: &'world mut World) -> Self;
}

pub struct SingleComponentIter<'world, C>
where
    C: Component,
{
    iter: std::slice::Iter<'world, Option<C>>,
}

impl<'world, C> FromWorld<'world> for SingleComponentIter<'world, C>
where
    C: Component,
{
    fn from_world(world: &'world mut World) -> Self {
        Self {
            iter: world
                .get_component_array::<C>()
                .map_or_else(|| [].iter(), |array| array.data_iter()),
        }
    }
}

impl<'world, C> Iterator for SingleComponentIter<'world, C>
where
    C: Component,
{
    type Item = &'world C;
    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            if item.is_some() {
                return item.as_ref();
            }
        }
        None
    }
}

pub struct SingleComponentIterMut<'world, C>
where
    C: Component,
{
    iter: std::slice::IterMut<'world, Option<C>>,
}

impl<'world, C> FromWorld<'world> for SingleComponentIterMut<'world, C>
where
    C: Component,
{
    fn from_world(world: &'world mut World) -> Self {
        Self {
            iter: world
                .get_component_array_mut::<C>()
                .map_or_else(|| [].iter_mut(), |array| array.data_iter_mut()),
        }
    }
}

impl<'world, C> Iterator for SingleComponentIterMut<'world, C>
where
    C: Component,
{
    type Item = &'world mut C;
    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            if item.is_some() {
                return item.as_mut();
            }
        }
        None
    }
}
