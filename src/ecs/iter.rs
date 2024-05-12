use atomic_refcell::{AtomicRefCell, AtomicRefMut};

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

pub struct PairComponentIter<'world, C1, C2> {
    array1: &'world [Option<C1>],
    array2: &'world [Option<C2>],
    index: usize,
}

impl<'world, C1, C2> FromWorld<'world> for PairComponentIter<'world, C1, C2>
where
    C1: Component,
    C2: Component,
{
    fn from_world(world: &'world mut World) -> Self {
        Self {
            array1: world
                .get_component_array::<C1>()
                .map_or_else(|| [].as_slice(), |array| array.data.as_slice()),
            array2: world
                .get_component_array::<C2>()
                .map_or_else(|| [].as_slice(), |array| array.data.as_slice()),
            index: 0,
        }
    }
}

impl<'world, C1, C2> Iterator for PairComponentIter<'world, C1, C2>
where
    C1: Component,
    C2: Component,
{
    type Item = (&'world C1, &'world C2);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let (Some(entry1), Some(entry2)) =
                (self.array1.get(self.index), self.array2.get(self.index))
            {
                if let (Some(elem1), Some(elem2)) = (entry1.as_ref(), entry2.as_ref()) {
                    self.index += 1;
                    return Some((elem1, elem2));
                }
            } else {
                // 最後まで探索した
                return None;
            }
            self.index += 1;
        }
    }
}

pub struct PairComponentIterMut<'world, C1, C2> {
    // 自己参照構造体にしたかった
    // UnsafeCellを使ったほうが良いかもしれない
    // TODO: 安全かどうか確認する
    _world: &'world mut World,
    array1: std::ptr::NonNull<Option<C1>>,
    array2: std::ptr::NonNull<Option<C2>>,
    len1: usize,
    len2: usize,
    index: usize,
}

impl<'world, C1, C2> FromWorld<'world> for PairComponentIterMut<'world, C1, C2>
where
    C1: Component,
    C2: Component,
{
    fn from_world(world: &'world mut World) -> Self {
        let array1 = world
            .get_component_array_mut::<C1>()
            .map_or_else(|| [].as_mut_slice(), |array| array.data.as_mut_slice());
        let len1 = array1.len();
        let array1 = std::ptr::NonNull::new(array1.as_mut_ptr()).unwrap();
        let array2 = world
            .get_component_array_mut::<C2>()
            .map_or_else(|| [].as_mut_slice(), |array| array.data.as_mut_slice());
        let len2 = array2.len();
        let array2 = std::ptr::NonNull::new(array2.as_mut_ptr()).unwrap();
        Self {
            _world: world,
            array1,
            array2,
            len1,
            len2,
            index: 0,
        }
    }
}

impl<'world, C1, C2> Iterator for PairComponentIterMut<'world, C1, C2>
where
    C1: Component,
    C2: Component,
{
    type Item = (&'world mut C1, &'world mut C2);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.len1 || self.index >= self.len2 {
                // 最後まで探索した
                return None;
            }
            let entry1 = unsafe { self.array1.as_ptr().add(self.index).as_mut().unwrap() };
            let entry2 = unsafe { self.array2.as_ptr().add(self.index).as_mut().unwrap() };
            if let (Some(elem1), Some(elem2)) = (entry1.as_mut(), entry2.as_mut()) {
                self.index += 1;
                return Some((elem1, elem2));
            }
            self.index += 1;
        }
    }
}
