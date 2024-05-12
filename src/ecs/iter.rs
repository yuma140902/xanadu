use std::cell::{Ref, RefMut};

use crate::collections::SparseVec;

use super::{Component, World};

pub trait FromWorld<'world> {
    fn from_world(world: &'world mut World) -> Self;
}

pub struct SingleComponentExclusiveIter<'world, C>
where
    C: Component,
{
    iter: std::slice::IterMut<'world, Option<C>>,
}

impl<'world, C> FromWorld<'world> for SingleComponentExclusiveIter<'world, C>
where
    C: Component,
{
    fn from_world(world: &'world mut World) -> Self {
        let iter = world
            .components
            .get_exclusive_iter_mut::<C>()
            .expect("Component not registered");
        Self { iter }
    }
}

impl<'world, C> Iterator for SingleComponentExclusiveIter<'world, C>
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

pub struct SingleComponentExclusiveIterMut<'world, C>
where
    C: Component,
{
    iter: std::slice::IterMut<'world, Option<C>>,
}

impl<'world, C> FromWorld<'world> for SingleComponentExclusiveIterMut<'world, C>
where
    C: Component,
{
    fn from_world(world: &'world mut World) -> Self {
        let iter = world
            .components
            .get_exclusive_iter_mut::<C>()
            .expect("Component not registered");
        Self { iter }
    }
}

impl<'world, C> Iterator for SingleComponentExclusiveIterMut<'world, C>
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
