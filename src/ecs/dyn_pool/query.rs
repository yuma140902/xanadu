use crate::generational_vec::GenerationalId;

pub trait Query {
    type Iter: Iterator<Item = GenerationalId>;
    fn iter_entities(&self) -> Self::Iter;
}
