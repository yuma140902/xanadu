use std::any::Any;

use crate::collections::TypedArray;

use super::Component;

pub struct AnyComponentArray {
    inner: Box<dyn Any>,
}

impl<T: Component> From<TypedArray<T>> for AnyComponentArray {
    fn from(value: TypedArray<T>) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl AnyComponentArray {
    pub fn downcast<T: Component>(&self) -> Option<&TypedArray<T>> {
        self.inner.downcast_ref()
    }

    pub fn downcast_mut<T: Component>(&mut self) -> Option<&mut TypedArray<T>> {
        self.inner.downcast_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn downcast() {
        let array = TypedArray::<i32>::new();
        let any_array = AnyComponentArray::from(array);
        assert!(any_array.downcast::<i32>().is_some());
        assert!(any_array.downcast::<f32>().is_none());
    }

    #[test]
    fn downcast_mut() {
        let array = TypedArray::<i32>::new();
        let mut any_array = AnyComponentArray::from(array);
        assert!(any_array.downcast_mut::<i32>().is_some());
        assert!(any_array.downcast_mut::<f32>().is_none());
    }
}
