pub mod pair;
pub mod single;

#[inline(always)]
#[cfg(not(target_arch = "wasm32"))]
pub fn black_box<T>(x: T) -> T {
    criterion::black_box(x)
}

#[inline(always)]
#[cfg(target_arch = "wasm32")]
pub fn black_box<T>(x: T) -> T {
    x
}
