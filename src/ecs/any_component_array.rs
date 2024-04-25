use std::any::Any;

use super::dyn_pool::ComponentArray;

pub struct AnyComponentArray {
    inner: Box<dyn Any>,
}

impl<T: bytemuck::Pod> From<ComponentArray<T>> for AnyComponentArray {
    fn from(value: ComponentArray<T>) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl AnyComponentArray {
    pub fn downcast<T: bytemuck::Pod>(&self) -> Option<&ComponentArray<T>> {
        self.inner.downcast_ref()
    }

    pub fn downcast_mut<T: bytemuck::Pod>(&mut self) -> Option<&mut ComponentArray<T>> {
        self.inner.downcast_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn downcast() {
        let array = ComponentArray::<i32>::new();
        let any_array = AnyComponentArray::from(array);
        assert!(any_array.downcast::<i32>().is_some());
        assert!(any_array.downcast::<f32>().is_none());
    }

    #[test]
    fn downcast_mut() {
        let array = ComponentArray::<i32>::new();
        let mut any_array = AnyComponentArray::from(array);
        assert!(any_array.downcast_mut::<i32>().is_some());
        assert!(any_array.downcast_mut::<f32>().is_none());
    }
}
