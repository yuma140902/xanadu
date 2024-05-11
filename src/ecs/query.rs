use super::{Component, World};

pub trait Query<'world, T> {
    type Iter: Iterator<Item = T>;
    fn get_iterator(world: &'world mut World) -> Self::Iter;
}

pub struct Mut<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<'world, C> Query<'world, &'world C> for C
where
    C: Component,
{
    type Iter = SingleRefQueryIter<'world, C>;

    fn get_iterator(world: &'world mut World) -> Self::Iter {
        SingleRefQueryIter {
            iter: world
                .get_component_array::<C>()
                .map_or_else(|| [].iter(), |array| array.data_iter()),
        }
    }
}

impl<'world, C> Query<'world, &'world mut C> for Mut<C>
where
    C: Component,
{
    type Iter = SingleMutQueryIter<'world, C>;

    fn get_iterator(world: &'world mut World) -> Self::Iter {
        SingleMutQueryIter {
            iter: world
                .get_component_array_mut::<C>()
                .map_or_else(|| [].iter_mut(), |array| array.data_iter_mut()),
        }
    }
}

pub struct SingleRefQueryIter<'a, C>
where
    C: Component,
{
    iter: std::slice::Iter<'a, Option<C>>,
}

impl<'a, C> Iterator for SingleRefQueryIter<'a, C>
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

pub struct SingleMutQueryIter<'a, C>
where
    C: Component,
{
    iter: std::slice::IterMut<'a, Option<C>>,
}

impl<'a, C> Iterator for SingleMutQueryIter<'a, C>
where
    C: Component,
{
    type Item = &'a mut C;

    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            if item.is_some() {
                return item.as_mut();
            }
        }
        None
    }
}
