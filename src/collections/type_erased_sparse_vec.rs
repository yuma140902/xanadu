use std::any::Any;

use crate::collections::SparseVec;

/// Type-erased version of [`SparseVec<T>`]
pub struct TypeErasedSparseVec {
    inner: Box<dyn Any>,
}

impl<T: 'static> From<SparseVec<T>> for TypeErasedSparseVec {
    fn from(value: SparseVec<T>) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl TypeErasedSparseVec {
    pub fn downcast<T: 'static>(&self) -> Option<&SparseVec<T>> {
        self.inner.downcast_ref()
    }

    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut SparseVec<T>> {
        self.inner.downcast_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn downcast() {
        let array = SparseVec::<i32>::new();
        let any_array = TypeErasedSparseVec::from(array);
        assert!(any_array.downcast::<i32>().is_some());
        assert!(any_array.downcast::<f32>().is_none());
    }

    #[test]
    fn downcast_mut() {
        let array = SparseVec::<i32>::new();
        let mut any_array = TypeErasedSparseVec::from(array);
        assert!(any_array.downcast_mut::<i32>().is_some());
        assert!(any_array.downcast_mut::<f32>().is_none());
    }
}
