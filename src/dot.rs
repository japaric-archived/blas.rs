//! dot := x^T * y

use {blasint, ffi};

/// The signature of `dot`
pub type Fn<T> = unsafe extern "C" fn (
    *const blasint,
    *const T,
    *const blasint,
    *const T,
    *const blasint,
) -> T;

impl ::Dot for f32 {
    fn dot() -> Fn<f32> {
        ffi::sdot_
    }
}

impl ::Dot for f64 {
    fn dot() -> Fn<f64> {
        ffi::ddot_
    }
}
