use std::collections::VecDeque;

/// Generational ID for the elements of [`GenerationalVec`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenerationalId {
    pub(crate) index: usize,
    generation: u32,
}

struct Entry<T> {
    generation: u32,
    inner: Option<T>,
}

/// Vec-like collection that supports generational IDs
pub struct GenerationalVec<T> {
    entries: Vec<Entry<T>>,
    // 空いているインデックスのキュー
    // 内容は必ずentriesの有効なインデックスである
    empty_queue: VecDeque<usize>,
}

impl<T> GenerationalVec<T> {
    /// Create a new empty collection
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            empty_queue: VecDeque::new(),
        }
    }

    /// Create a new empty collection with a specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            empty_queue: VecDeque::new(),
        }
    }

    /// Remove an element from the collection
    pub fn remove(&mut self, id: GenerationalId) -> Option<T> {
        if let Some(entry) = self.entries.get_mut(id.index) {
            if entry.generation == id.generation {
                let value = entry.inner.take();
                self.empty_queue.push_back(id.index);
                return value;
            }
        }
        None
    }

    /// Add an element to the collection
    pub fn add(&mut self, value: T) -> GenerationalId {
        if let Some(index) = self.empty_queue.pop_front() {
            let generation = self.entries[index].generation + 1;
            self.entries[index].generation = generation;
            self.entries[index].inner = Some(value);
            GenerationalId { index, generation }
        } else {
            let generation = 0;
            let index = self.entries.len();
            self.entries.push(Entry {
                generation: 0,
                inner: Some(value),
            });
            GenerationalId { index, generation }
        }
    }

    /// Get a reference to an element
    pub fn get(&self, id: GenerationalId) -> Option<&T> {
        if let Some(entry) = self.entries.get(id.index) {
            if entry.generation == id.generation {
                return entry.inner.as_ref();
            }
        }
        None
    }

    /// Get a mutable reference to an element
    pub fn get_mut(&mut self, id: GenerationalId) -> Option<&mut T> {
        if let Some(entry) = self.entries.get_mut(id.index) {
            if entry.generation == id.generation {
                return entry.inner.as_mut();
            }
        }
        None
    }

    /// Get the number of elements in the collection
    pub fn len(&self) -> usize {
        self.entries.len() - self.empty_queue.len()
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Default for GenerationalVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn remove() {
        let mut v = GenerationalVec::new();
        let id1 = v.add(100);
        let id2 = v.add(200);
        let id3 = v.add(300);
        assert_eq!(v.remove(id2), Some(200));
        assert_eq!(v.remove(id1), Some(100));
        assert_eq!(v.remove(id3), Some(300));
    }

    #[test]
    fn get() {
        let mut v = GenerationalVec::new();
        let id1 = v.add(100);
        let id2 = v.add(200);
        let id3 = v.add(300);
        assert_eq!(v.get(id1), Some(&100));
        assert_eq!(v.get(id2), Some(&200));
        assert_eq!(v.get(id3), Some(&300));
    }

    #[test]
    fn get_mut() {
        let mut v = GenerationalVec::new();
        let id1 = v.add(100);
        let id2 = v.add(200);
        let id3 = v.add(300);
        *v.get_mut(id1).unwrap() += 1;
        *v.get_mut(id2).unwrap() += 1;
        *v.get_mut(id3).unwrap() += 1;
        assert_eq!(v.get(id1), Some(&101));
        assert_eq!(v.get(id2), Some(&201));
        assert_eq!(v.get(id3), Some(&301));
    }

    #[test]
    fn num_elements() {
        let mut v = GenerationalVec::new();
        assert_eq!(v.len(), 0);
        let id1 = v.add(10);
        let id2 = v.add(20);
        assert_eq!(v.len(), 2);
        v.remove(id2);
        assert_eq!(v.len(), 1);
        v.remove(id1);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn remove_and_then_add() {
        let mut v = GenerationalVec::new();
        v.add(100);
        let id2 = v.add(200);
        v.add(300);
        assert_eq!(v.len(), 3);
        assert_eq!(v.entries.len(), 3);

        v.remove(id2);
        assert_eq!(v.len(), 2);
        assert_eq!(v.entries.len(), 3);

        v.add(400);
        assert_eq!(v.len(), 3);
        assert_eq!(v.entries.len(), 3);

        v.add(500);
        assert_eq!(v.len(), 4);
        assert_eq!(v.entries.len(), 4);
    }

    #[test]
    fn len() {
        let v = GenerationalVec::<()>::new();
        assert_eq!(v.len(), 0);
        assert!(v.is_empty());
    }

    #[test]
    fn empty_after_remove() {
        let mut v = GenerationalVec::new();
        let id = v.add(100);
        assert!(!v.is_empty());
        v.remove(id);
        assert!(v.is_empty());
    }
}
