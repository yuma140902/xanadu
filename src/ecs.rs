use std::alloc::Layout;

#[derive(Debug)]
pub struct ComponentArray {
    ptr: *mut u8,
    layout: Layout,
    len: usize,
    type_id: std::any::TypeId,
    element_size: usize,
    capacity: usize,
}

impl ComponentArray {
    /// ## Panics
    ///
    /// - メモリ確保に失敗した場合にpanicする
    /// - `T`が[`Layout::from_size_align()`]の事前条件を満たさなかった場合にpanicする
    pub fn new<T>() -> Self
    where
        T: Sized + bytemuck::Pod + Default,
    {
        let type_id = std::any::TypeId::of::<T>();
        let alignment = std::mem::align_of::<T>();
        let size = std::mem::size_of::<T>();
        let capacity = 0;
        let layout = Layout::from_size_align(capacity * size, alignment).unwrap();
        let ptr = unsafe {
            let ptr = std::alloc::alloc(layout);
            assert!(!ptr.is_null(), "Failed to allocate memory");
            ptr
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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn type_id(&self) -> std::any::TypeId {
        self.type_id
    }

    /// 要素を追加する。必要に応じてメモリを再確保する。
    ///
    /// ## Safety
    ///
    /// - `T`は[`Self::new()`]で指定した型と同じでなければならない
    ///
    /// ## Panics
    ///
    /// - メモリ確保に失敗した場合にpanicする
    /// - `T`が[`Layout::from_size_align()`]の事前条件を満たさなかった場合にpanicする
    pub unsafe fn add_unchecked<T: bytemuck::Pod>(&mut self, value: T) {
        if self.len >= self.capacity {
            println!("realloc");
            let new_capacity = if self.capacity == 0 {
                1
            } else {
                self.capacity * 2
            };
            let new_layout =
                Layout::from_size_align(new_capacity * self.element_size, self.layout.align())
                    .unwrap();
            let new_ptr = std::alloc::realloc(self.ptr, self.layout, new_layout.size());
            assert!(!new_ptr.is_null(), "Failed to reallocate memory");
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
    /// - `T`は[`Self::new()`]で指定した型と同じでなければならない
    /// - `index`は0以上[`Self::len()`]未満でなければならない
    pub unsafe fn get_ptr<T: bytemuck::Pod>(&self, index: usize) -> *mut T {
        let ptr = self.ptr.cast::<T>();
        ptr.add(index)
    }

    /// ## Safety
    ///
    /// - `T`は[`Self::new()`]で指定した型と同じでなければならない
    /// - `index`は0以上[`Self::len()`]未満でなければならない
    pub unsafe fn get_unchecked<T: bytemuck::Pod>(&self, index: usize) -> &T {
        &*self.get_ptr(index)
    }

    /// ## Safety
    ///
    /// - `T`は[`Self::new()`]で指定した型と同じでなければならない
    /// - `index`は0以上[`Self::len()`]未満でなければならない
    pub unsafe fn get_mut_unchecked<T: bytemuck::Pod>(&mut self, index: usize) -> &mut T {
        &mut *self.get_ptr(index)
    }

    /// 要素を追加する。必要に応じてメモリを再確保する。
    ///
    /// ## Panics
    ///
    /// `T`が[`Self::new()`]で指定した型に一致しなかった場合にpanicする
    pub fn add<T: bytemuck::Pod>(&mut self, value: T)
    where
        T: Sized + bytemuck::Pod + Default,
    {
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

impl Drop for ComponentArray {
    fn drop(&mut self) {
        unsafe {
            #[cfg(debug_assertions)]
            {
                // use-after-freeバグがあった場合にすぐに気づくことができるように適当な値(0xFF)で埋める
                std::ptr::write_bytes(self.ptr, 0xFF, self.len * self.element_size);
            }
            std::alloc::dealloc(self.ptr, self.layout);
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
        let _ = ComponentArray::new::<u32>();
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
        let mut ca = ComponentArray::new::<Position>();

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
        let mut ca = ComponentArray::new::<Position>();

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
        let mut ca = ComponentArray::new::<Position>();

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
        let mut ca = ComponentArray::new::<Position>();

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
        let mut ca = ComponentArray::new::<Position>();

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
        let mut ca = ComponentArray::new::<Position>();

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
        let mut ca = ComponentArray::new::<Position>();
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
        let mut ca = ComponentArray::new::<Position>();
        ca.add(Position {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let elem = ca.get::<f64>(0);
        assert_eq!(elem, None);
    }
}
