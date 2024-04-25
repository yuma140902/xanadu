use crate::generational_vec::{GenerationalId, GenerationalVec};

pub struct WorldBuilder {
    world: World,
}

impl WorldBuilder {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
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
}

impl World {
    pub(self) fn new() -> Self {
        Self {
            entities: GenerationalVec::new(),
        }
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
    #[test]
    fn new() {
        let world = super::World::new();
        assert_eq!(world.entities.len(), 0);
    }

    #[test]
    fn new_entity() {
        let mut world = super::World::new();
        let entity = world.new_entity();
        assert_eq!(world.entities.len(), 1);
        assert_eq!(world.entities.get(entity), Some(&()));
    }
}
