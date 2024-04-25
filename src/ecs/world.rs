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

    pub fn register_component<T: bytemuck::Pod>(&mut self) -> &mut Self {
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

    pub fn new_entity(&mut self) -> GenerationalId {
        self.entities.add(())
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
}
