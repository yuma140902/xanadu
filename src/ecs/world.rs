use std::{any::TypeId, collections::HashMap};

use crate::generational_vec::{GenerationalId, GenerationalVec};

use super::dyn_pool::{AnyComponentArray, ComponentArray};

pub struct WorldBuilder {
    world: World,
}

impl WorldBuilder {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn register_component<T: bytemuck::Pod>(mut self) -> Self {
        self.world.register_component::<T>();
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
    component_arrays: HashMap<TypeId, AnyComponentArray>,
}

impl World {
    pub(self) fn new() -> Self {
        Self {
            entities: GenerationalVec::new(),
            component_arrays: HashMap::new(),
        }
    }

    pub(self) fn register_component<T: bytemuck::Pod>(&mut self) {
        self.component_arrays
            .insert(TypeId::of::<T>(), ComponentArray::<T>::new().into());
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
    pub fn attach_component<T: bytemuck::Pod>(
        &mut self,
        entity: GenerationalId,
        component: T,
    ) -> Option<T> {
        if let Some(array) = self.component_arrays.get_mut(&TypeId::of::<T>()) {
            if let Some(array) = array.downcast_mut::<T>() {
                return array.replace(entity.index, component);
            }
        }
        None
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let world = World::new();
        assert!(world.entities.is_empty());
        assert!(world.component_arrays.is_empty());
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
