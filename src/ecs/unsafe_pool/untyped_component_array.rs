use std::{alloc::Layout, ptr::NonNull};

#[derive(Debug)]
pub struct UntypedComponentArray {
    ptr: NonNull<u8>,
    layout: Layout,
    len: usize,
    type_id: std::any::TypeId,
    element_size: usize,
    capacity: usize,
}

impl UntypedComponentArray {
    /// ## Panics
    ///
    /// - アロケーターがnullを返した場合にpanicする
    /// - `T`が[`Layout::from_size_align()`]の事前条件を満たさなかった場合にpanicする
    pub fn new<T: bytemuck::Pod>() -> Self {
        Self::with_capacity::<T>(0)
    }

    /// ## Panics
    ///
    /// - アロケーターがnullを返した場合にpanicする
    /// - `T`が[`Layout::from_size_align()`]の事前条件を満たさなかった場合にpanicする
    pub fn with_capacity<T: bytemuck::Pod>(capacity: usize) -> Self {
        let type_id = std::any::TypeId::of::<T>();
        let alignment = std::mem::align_of::<T>();
        let size = std::mem::size_of::<T>();
        let alloc_size = (capacity * size).max(alignment).max(1);
        let layout = Layout::from_size_align(alloc_size, alignment).unwrap();
        let ptr = unsafe {
            let ptr = std::alloc::alloc(layout);
            NonNull::new(ptr).expect("Failed to allocate memory")
        };
        Self {
            type_id,
            ptr,
            layout,
            len: 0,
            element_size: size,
            capacity,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn type_id(&self) -> std::any::TypeId {
        self.type_id
    }

    /// 要素を追加する。必要に応じてメモリを再確保する。
    ///
    /// ## Safety
    ///
    /// - `T`はインスタンスの生成時に指定した型と同じでなければならない
    ///
    /// ## Panics
    ///
    /// - アロケーターがnullを返した場合にpanicする
    /// - `T`が[`Layout::from_size_align()`]の事前条件を満たさなかった場合にpanicする
    pub unsafe fn add_unchecked<T: bytemuck::Pod>(&mut self, value: T) {
        if self.len >= self.capacity {
            println!("realloc");
            let new_capacity = if self.capacity == 0 {
                1
            } else {
                self.capacity * 2
            };
            let new_alloc_size = (new_capacity * self.element_size)
                .max(self.layout.align())
                .max(self.layout.size())
                .max(1);
            let new_layout = Layout::from_size_align(new_alloc_size, self.layout.align()).unwrap();
            let new_ptr = std::alloc::realloc(self.ptr.as_ptr(), self.layout, new_layout.size());
            let new_ptr = NonNull::new(new_ptr).expect("Failed to reallocate memory");
            self.ptr = new_ptr;
            self.layout = new_layout;
            self.capacity = new_capacity;
        }
        let index = self.len;
        self.len += 1;
        let ptr = self.get_ptr::<T>(index);
        ptr.write(value);
    }

    /// ## Safety
    ///
    /// - `T`はインスタンスの生成時に指定した型と同じでなければならない
    /// - `index`は0以上[`Self::len()`]未満でなければならない
    pub const unsafe fn get_ptr<T: bytemuck::Pod>(&self, index: usize) -> *mut T {
        // https://github.com/rust-lang/rust/issues/117691
        // の変更がリリースされたらNonNull.addを使うようにする
        let ptr = self.ptr.as_ptr().cast::<T>();
        ptr.add(index)
    }

    /// ## Safety
    ///
    /// - `T`はインスタンスの生成時に指定した型と同じでなければならない
    /// - `index`は0以上[`Self::len()`]未満でなければならない
    #[allow(clippy::missing_const_for_fn)]
    pub unsafe fn get_unchecked<T: bytemuck::Pod>(&self, index: usize) -> &T {
        &*self.get_ptr(index)
    }

    /// ## Safety
    ///
    /// - `T`はインスタンスの生成時に指定した型と同じでなければならない
    /// - `index`は0以上[`Self::len()`]未満でなければならない
    pub unsafe fn get_mut_unchecked<T: bytemuck::Pod>(&mut self, index: usize) -> &mut T {
        &mut *self.get_ptr(index)
    }

    /// 要素を追加する。必要に応じてメモリを再確保する。
    ///
    /// ## Panics
    ///
    /// `T`がインスタンスの生成時に指定した型と一致しなかった場合にpanicする
    pub fn add<T: bytemuck::Pod>(&mut self, value: T) {
        assert!(self.type_id == std::any::TypeId::of::<T>(), "Type mismatch");
        unsafe {
            self.add_unchecked(value);
        }
    }

    pub fn get<T: bytemuck::Pod>(&self, index: usize) -> Option<&T> {
        if index >= self.len || self.type_id != std::any::TypeId::of::<T>() {
            return None;
        }
        unsafe { Some(self.get_unchecked(index)) }
    }

    pub fn get_mut<T: bytemuck::Pod>(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len || self.type_id != std::any::TypeId::of::<T>() {
            return None;
        }
        unsafe { Some(self.get_mut_unchecked(index)) }
    }
}

impl Drop for UntypedComponentArray {
    fn drop(&mut self) {
        unsafe {
            #[cfg(debug_assertions)]
            {
                // use-after-freeバグがあった場合にすぐに気づくことができるように適当な値(0xFF)で埋める
                std::ptr::write_bytes(self.ptr.as_ptr(), 0xFF, self.len * self.element_size);
            }
            std::alloc::dealloc(self.ptr.as_ptr(), self.layout);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[cfg(all(target_arch = "wasm32", feature = "test_in_browser"))]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn new() {
        let _ = UntypedComponentArray::new::<u32>();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn with_capacity() {
        let _ = UntypedComponentArray::with_capacity::<u32>(10);
    }

    #[repr(C)]
    #[derive(Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
    struct Position {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_and_get_unsafe() {
        let mut ca = UntypedComponentArray::new::<Position>();

        unsafe {
            ca.add_unchecked(Position {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            });
            ca.add_unchecked(Position {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            });

            let elem0 = ca.get_unchecked::<Position>(0);
            assert_eq!(
                elem0,
                &Position {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0
                }
            );
            let elem1 = ca.get_unchecked::<Position>(1);
            assert_eq!(
                elem1,
                &Position {
                    x: 4.0,
                    y: 5.0,
                    z: 6.0
                }
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_and_get() {
        let mut ca = UntypedComponentArray::new::<Position>();

        ca.add(Position {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        ca.add(Position {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        });

        let elem0 = ca.get::<Position>(0);
        assert_eq!(
            elem0,
            Some(&Position {
                x: 1.0,
                y: 2.0,
                z: 3.0
            })
        );
        let elem1 = ca.get::<Position>(1);
        assert_eq!(
            elem1,
            Some(&Position {
                x: 4.0,
                y: 5.0,
                z: 6.0
            })
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_and_get_mut_field_unsafe() {
        let mut ca = UntypedComponentArray::new::<Position>();

        unsafe {
            ca.add_unchecked(Position {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            });
            ca.add_unchecked(Position {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            });

            let elem1_mut = ca.get_mut_unchecked::<Position>(1);
            elem1_mut.x = 7.0;

            let elem1 = ca.get_unchecked::<Position>(1);
            assert_eq!(
                elem1,
                &Position {
                    x: 7.0,
                    y: 5.0,
                    z: 6.0
                }
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_and_get_mut_self_unsafe() {
        let mut ca = UntypedComponentArray::new::<Position>();

        unsafe {
            ca.add_unchecked(Position {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            });

            ca.add_unchecked(Position {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            });

            let elem1_mut = ca.get_mut_unchecked::<Position>(1);
            *elem1_mut = Position {
                x: 40.0,
                y: 50.0,
                z: 60.0,
            };

            let elem1 = ca.get_unchecked::<Position>(1);
            assert_eq!(
                elem1,
                &Position {
                    x: 40.0,
                    y: 50.0,
                    z: 60.0
                }
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_and_get_mut_field() {
        let mut ca = UntypedComponentArray::new::<Position>();

        ca.add(Position {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        ca.add(Position {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        });

        let mut elem1_mut = ca.get_mut::<Position>(1);
        elem1_mut.as_mut().unwrap().x = 7.0;

        let elem1 = ca.get::<Position>(1);
        assert_eq!(
            elem1,
            Some(&Position {
                x: 7.0,
                y: 5.0,
                z: 6.0
            })
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_and_get_mut_self() {
        let mut ca = UntypedComponentArray::new::<Position>();

        ca.add(Position {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });

        ca.add(Position {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        });

        let elem1_mut = ca.get_mut::<Position>(1).unwrap();
        *elem1_mut = Position {
            x: 40.0,
            y: 50.0,
            z: 60.0,
        };

        let elem1 = ca.get::<Position>(1);
        assert_eq!(
            elem1,
            Some(&Position {
                x: 40.0,
                y: 50.0,
                z: 60.0
            })
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_out_of_range() {
        let mut ca = UntypedComponentArray::new::<Position>();
        ca.add(Position {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let elem1 = ca.get::<Position>(1);
        assert_eq!(elem1, None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_wrong_type() {
        let mut ca = UntypedComponentArray::new::<Position>();
        ca.add(Position {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let elem = ca.get::<f64>(0);
        assert_eq!(elem, None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn multiple_array() {
        let mut c1 = UntypedComponentArray::new::<Position>();
        let mut c2 = UntypedComponentArray::new::<Position>();

        c1.add(Position {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });

        c2.add(Position {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        });

        let elem1 = c1.get::<Position>(0);
        assert_eq!(
            elem1,
            Some(&Position {
                x: 1.0,
                y: 2.0,
                z: 3.0
            })
        );

        let elem2 = c2.get::<Position>(0);
        assert_eq!(
            elem2,
            Some(&Position {
                x: 4.0,
                y: 5.0,
                z: 6.0
            })
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn empty_type() {
        let mut ca = UntypedComponentArray::new::<()>();
        ca.add(());
        let elem = ca.get::<()>(0);
        assert_eq!(elem, Some(&()));
    }
}
