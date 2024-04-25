pub struct ComponentArray<T> {
    data: Vec<Option<T>>,
}

impl<T: bytemuck::Pod> ComponentArray<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn insert(&mut self, index: usize, component: T) {
        if index >= self.data.len() {
            self.data.resize_with(index + 1, || None);
        }
        self.data[index] = Some(component);
    }

    pub fn remove(&mut self, index: usize) {
        if let Some(component) = self.data.get_mut(index) {
            *component = None;
        }
    }
}

impl<T: bytemuck::Pod> Default for ComponentArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn insert() {
        let mut array = ComponentArray::new();
        array.insert(0, 42);
        assert_eq!(array.data.len(), 1);
        assert_eq!(array.data[0], Some(42));
    }

    #[test]
    fn insert_offset() {
        let mut array = ComponentArray::new();
        array.insert(0, 42);
        array.insert(2, 43);
        assert_eq!(array.data.len(), 3);
        assert_eq!(array.data[0], Some(42));
        assert_eq!(array.data[1], None);
        assert_eq!(array.data[2], Some(43));
    }

    #[test]
    fn remove() {
        let mut array = ComponentArray::new();
        array.insert(0, 42);
        array.remove(0);
        assert_eq!(array.data.len(), 1);
        assert_eq!(array.data[0], None);
    }
}
