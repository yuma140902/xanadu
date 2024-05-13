use std::cell::{Ref, RefMut};

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

pub struct SingleComponentRefIter<'world, C>
where
    C: Component,
{
    slice: Option<Ref<'world, [Option<C>]>>,
}

impl<'world, C> FromWorld<'world> for SingleComponentRefIter<'world, C>
where
    C: Component,
{
    fn from_world(world: &'world mut World) -> Self {
        let slice = world
            .components
            .borrow_slice::<C>()
            .expect("Component not registered");
        Self { slice: Some(slice) }
    }
}

impl<'world, C> Iterator for SingleComponentRefIter<'world, C>
where
    C: Component,
{
    type Item = Ref<'world, C>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.slice.take() {
                Some(borrow) => match *borrow {
                    [] => return None,
                    [_, ..] => {
                        let (head, tail) = Ref::map_split(borrow, |slice| (&slice[0], &slice[1..]));
                        self.slice.replace(tail);
                        if head.is_some() {
                            return Some(Ref::map(head, |v| {
                                // SAFETY: head is Some
                                unsafe { v.as_ref().unwrap_unchecked() }
                            }));
                        } else {
                            continue;
                        }
                    }
                },
                None => return None,
            }
        }
    }
}

pub struct SingleComponentRefIterMut<'world, C>
where
    C: Component,
{
    slice: Option<RefMut<'world, [Option<C>]>>,
}

impl<'world, C> FromWorld<'world> for SingleComponentRefIterMut<'world, C>
where
    C: Component,
{
    fn from_world(world: &'world mut World) -> Self {
        let slice = world
            .components
            .borrow_mut_slice::<C>()
            .expect("Component not registered");
        Self { slice: Some(slice) }
    }
}

impl<'world, C> Iterator for SingleComponentRefIterMut<'world, C>
where
    C: Component,
{
    type Item = RefMut<'world, C>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.slice.take() {
                Some(borrow) => match *borrow {
                    [] => return None,
                    [_, ..] => {
                        let (head, tail) = RefMut::map_split(borrow, |slice| {
                            let (left, right) = slice.split_at_mut(1);
                            (&mut left[0], right)
                        });
                        self.slice.replace(tail);
                        if head.is_some() {
                            return Some(RefMut::map(head, |v| {
                                // SAFETY: head is Some
                                unsafe { v.as_mut().unwrap_unchecked() }
                            }));
                        } else {
                            continue;
                        }
                    }
                },
                None => return None,
            }
        }
    }
}
