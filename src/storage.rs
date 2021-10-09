use std::collections::HashMap;

use super::Entity;

pub trait Storage<T> {
    fn new() -> Self;
    fn get_component(&self, entity: &Entity) -> Option<&T>;
    fn get_component_mut(&mut self, entity: &Entity) -> Option<&mut T>;
    fn set_component(&mut self, entity: &Entity, component: T);
}

pub struct VecStorage<T> {
    entity_to_index: HashMap<Entity, usize>,
    components: Vec<T>,
}

impl<T> Storage<T> for VecStorage<T> {
    fn new() -> Self {
        Self {
            entity_to_index: HashMap::new(),
            components: Vec::new(),
        }
    }

    fn get_component(&self, entity: &Entity) -> Option<&T> {
        let index = self.entity_to_index.get(entity)?;
        self.components.get(*index)
    }

    fn get_component_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        let index = self.entity_to_index.get(entity)?;
        self.components.get_mut(*index)
    }

    fn set_component(&mut self, entity: &Entity, component: T) {
        self.components.push(component);
        self.entity_to_index
            .insert(*entity, self.components.len() - 1);
    }
}

pub struct HashMapStorage<T> {
    components: HashMap<Entity, T>,
}

impl<T> Storage<T> for HashMapStorage<T> {
    fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    fn get_component(&self, entity: &Entity) -> Option<&T> {
        self.components.get(entity)
    }

    fn get_component_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.components.get_mut(entity)
    }

    fn set_component(&mut self, entity: &Entity, component: T) {
        self.components.insert(*entity, component);
    }
}
