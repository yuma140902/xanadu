use crate::generational_vec::GenerationalVec;

pub struct World {
    entities: GenerationalVec<()>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: GenerationalVec::new(),
        }
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
}
