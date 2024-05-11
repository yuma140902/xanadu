use std::slice;

/// Vec-like collection that supports sparse indices
pub struct SparseVec<T> {
    data: Vec<Option<T>>,
}

impl<T> SparseVec<T> {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// 指定されたインデックスに要素を追加する。必要に応じて配列を伸ばす。
    ///
    /// ## Returns
    ///
    /// 以前の要素があればそれを返す。なければNoneを返す
    pub fn replace(&mut self, index: usize, component: T) -> Option<T> {
        if index >= self.data.len() {
            self.data.resize_with(index + 1, || None);
        }
        self.data[index].replace(component)
    }

    /// 指定した要素を取得する。
    ///
    /// TODO: テスト
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index).and_then(|v| v.as_ref())
    }

    /// 指定した要素を削除する。
    ///
    /// ## Returns
    ///
    /// 以前の要素があればそれをを返す。なければNoneを返す
    ///
    /// TODO: テスト
    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.data.get_mut(index).and_then(|v| v.take())
    }

    /// 有効な要素のイテレータを返す
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().filter_map(|v| v.as_ref())
    }

    /// 内部の配列のイテレータを返す
    pub fn data_iter(&self) -> slice::Iter<'_, Option<T>> {
        self.data.iter()
    }

    /// 有効な要素の可変イテレータを返す
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().filter_map(|v| v.as_mut())
    }

    /// 内部の配列の可変イテレータを返す
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
