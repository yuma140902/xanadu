use std::slice;

/// Vec-like collection that supports sparse indices
pub struct SparseVec<T> {
    data: Vec<Option<T>>,
}

impl<T> SparseVec<T> {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Add an element to the collection at the specified index. Extend the array as needed.
    ///
    /// # Returns
    ///
    /// Returns the previous element, if any. Otherwise, returns None.
    pub fn replace(&mut self, index: usize, component: T) -> Option<T> {
        if index >= self.data.len() {
            self.data.resize_with(index + 1, || None);
        }
        self.data[index].replace(component)
    }

    /// Get an element at the specified index.
    ///
    // TODO: テスト
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index).and_then(|v| v.as_ref())
    }

    /// Remove an element at the specified index, if one exists.
    ///
    /// # Returns
    ///
    /// Returns the previous element, if any. Otherwise, returns None.
    ///
    // TODO: テスト
    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.data.get_mut(index).and_then(|v| v.take())
    }

    /// Returns an iterator over the elements.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().filter_map(|v| v.as_ref())
    }

    /// Returns an iterator over the internal array.
    pub fn data_iter(&self) -> slice::Iter<'_, Option<T>> {
        self.data.iter()
    }

    /// Returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().filter_map(|v| v.as_mut())
    }

    /// Returns a mutable iterator over the internal array.
    pub fn data_iter_mut(&mut self) -> slice::IterMut<'_, Option<T>> {
        self.data.iter_mut()
    }
}

impl<T> Default for SparseVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn replace() {
        let mut array = SparseVec::new();
        array.replace(0, 42);
        assert_eq!(array.data.len(), 1);
        assert_eq!(array.data[0], Some(42));
    }

    #[test]
    fn replace_offset() {
        let mut array = SparseVec::new();
        array.replace(0, 42);
        array.replace(2, 43);
        assert_eq!(array.data.len(), 3);
        assert_eq!(array.data[0], Some(42));
        assert_eq!(array.data[1], None);
        assert_eq!(array.data[2], Some(43));
    }

    #[test]
    fn replace_return_value() {
        let mut array = SparseVec::new();
        assert_eq!(array.replace(0, 42), None);
        assert_eq!(array.replace(0, 43), Some(42));
    }
}
