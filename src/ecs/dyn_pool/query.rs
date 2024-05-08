use super::{Component, World};

pub trait Query<'world, T> {
    type Iter: Iterator<Item = T>;
    fn get_iterator(&self, world: &'world World) -> Self::Iter;
}

impl<'world, C> Query<'world, &'world C> for C
where
    C: Component,
{
    type Iter = SingleQueueIter<'world, C>;

    fn get_iterator(&self, world: &'world World) -> Self::Iter {
        if let Some(array) = world.get_component_array::<C>() {
            SingleQueueIter {
                iter: array.data_iter(),
            }
        } else {
            SingleQueueIter { iter: [].iter() }
        }
    }
}

pub struct SingleQueueIter<'a, C>
where
    C: Component,
{
    iter: std::slice::Iter<'a, Option<C>>,
}

impl<'a, C> Iterator for SingleQueueIter<'a, C>
where
    C: Component,
{
    type Item = &'a C;
    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            if item.is_some() {
                return item.as_ref();
            }
        }
        None
    }
}
