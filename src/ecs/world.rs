use std::{any::TypeId, collections::HashMap};

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
        if let Some(array) = self.components.get_mut::<T>() {
            return array.replace(entity.index, component);
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
    map: HashMap<TypeId, TypeErasedSparseVec>,
}

impl Components {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn register<T: Component>(&mut self) {
        self.map
            .insert(TypeId::of::<T>(), SparseVec::<T>::new().into());
    }

    pub(crate) fn get<T: Component>(&self) -> Option<&SparseVec<T>> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|any_array| any_array.downcast::<T>())
    }

    pub(crate) fn get_mut<T: Component>(&mut self) -> Option<&mut SparseVec<T>> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|any_array| any_array.downcast_mut::<T>())
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
