//! dot := x^T * y

use complex::Complex;

use {blasint, ffi};

/// The signature of `dot`
pub type Fn<T> = unsafe extern "C" fn (
    *const blasint,
    *const T,
    *const blasint,
    *const T,
    *const blasint,
) -> T;

impl ::Dot for Complex<f32> {
    fn dot() -> Fn<Complex<f32>> {
        ffi::cdotu_
    }
}

impl ::Dot for Complex<f64> {
    fn dot() -> Fn<Complex<f64>> {
        ffi::zdotu_
    }
}

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
