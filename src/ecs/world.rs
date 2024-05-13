use std::{
    any::TypeId,
    borrow::BorrowMut,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use crate::collections::{GenerationalId, GenerationalVec, SparseVec, TypeErasedSparseVec};

use super::{Component, System};

pub struct WorldBuilder {
    world: World,
}

impl WorldBuilder {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn register_component<T: Component>(mut self) -> Self {
        self.world.components.register::<T>();
        self
    }

    pub fn build(self) -> World {
        self.world
    }
}

impl Default for WorldBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct World {
    entities: GenerationalVec<()>,
    pub(crate) components: Components,
}

impl World {
    pub(self) fn new() -> Self {
        Self {
            entities: GenerationalVec::new(),
            components: Components::new(),
        }
    }

    pub fn builder() -> WorldBuilder {
        WorldBuilder::new()
    }

    pub fn new_entity(&mut self) -> GenerationalId {
        self.entities.add(())
    }

    /// エンティティにコンポーネントを追加する
    ///
    ///
    /// ## Returns
    ///
    /// 以前のコンポーネントがあればそれを返す。なければNoneを返す
    pub fn attach_component<T: Component>(
        &mut self,
        entity: GenerationalId,
        component: T,
    ) -> Option<T> {
        if let Some(mut array) = self.components.borrow_mut::<T>() {
            return array.borrow_mut().replace(entity.index, component);
        }
        None
    }

    pub fn execute<'world, T>(&'world mut self, system: impl System<'world, T>) {
        system.execute(self);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Components {
    map: HashMap<TypeId, RefCell<TypeErasedSparseVec>>,
}

impl Components {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn register<T: Component>(&mut self) {
        self.map.insert(
            TypeId::of::<T>(),
            RefCell::new(SparseVec::<T>::new().into()),
        );
    }

    pub(crate) fn get_exclusive_iter_mut<T: Component>(
        &mut self,
    ) -> Option<std::slice::IterMut<'_, Option<T>>> {
        let refcell = self.map.get_mut(&TypeId::of::<T>())?;
        let optional_vec = refcell.get_mut().downcast_mut::<T>();
        // SAFETY:
        // self.map[TypeId::of<T>] には SparseVec<T> が登録されているので、ダウンキャストは必ず成功する
        let vec = unsafe { optional_vec.unwrap_unchecked() };
        Some(vec.data_iter_mut())
    }

    pub(crate) fn borrow_slice<T: Component>(&self) -> Option<Ref<[Option<T>]>> {
        let refcell = self.map.get(&TypeId::of::<T>())?;
        let slice = Ref::map(refcell.borrow(), |vec| {
            // SAFETY:
            // self.map[TypeId::of<T>] には SparseVec<T> が登録されているので、ダウンキャストは必ず成功する
            unsafe { vec.downcast::<T>().unwrap_unchecked() }.data_slice()
        });
        Some(slice)
    }

    pub(crate) fn borrow_mut_slice<T: Component>(&self) -> Option<RefMut<[Option<T>]>> {
        let refcell = self.map.get(&TypeId::of::<T>())?;
        let slice = RefMut::map(refcell.borrow_mut(), |vec| {
            // SAFETY:
            // self.map[TypeId::of<T>] には SparseVec<T> が登録されているので、ダウンキャストは必ず成功する
            unsafe { vec.downcast_mut::<T>().unwrap_unchecked() }.data_mut_slice()
        });
        Some(slice)
    }

    pub(crate) fn borrow_mut<T: Component>(&self) -> Option<RefMut<SparseVec<T>>> {
        let refcell = self.map.get(&TypeId::of::<T>())?;
        let vec = RefMut::map(refcell.borrow_mut(), |vec| {
            // SAFETY:
            // self.map[TypeId::of<T>] には SparseVec<T> が登録されているので、ダウンキャストは必ず成功する
            unsafe { vec.downcast_mut::<T>().unwrap_unchecked() }
        });
        Some(vec)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let world = World::new();
        assert!(world.entities.is_empty());
        assert!(world.components.map.is_empty());
    }

    #[test]
    fn new_entity() {
        let mut world = World::new();
        let entity = world.new_entity();
        assert_eq!(world.entities.len(), 1);
        assert_eq!(world.entities.get(entity), Some(&()));
    }

    #[test]
    fn attach_component_return_value() {
        let mut world = World::builder().register_component::<i32>().build();
        let entity = world.new_entity();
        assert_eq!(world.attach_component(entity, 42), None);
        assert_eq!(world.attach_component(entity, 43), Some(42));
    }
}
