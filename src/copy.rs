//! y := x

use complex::Complex;

use {blasint, ffi};

/// The signature of `copy`
pub type Fn<T> = unsafe extern "C" fn (
    *const blasint,
    *const T,
    *const blasint,
    *mut T,
    *const blasint,
);

impl ::Copy for Complex<f32> {
    fn copy() -> Fn<Complex<f32>> {
        ffi::ccopy_
    }
}

impl ::Copy for Complex<f64> {
    fn copy() -> Fn<Complex<f64>> {
        ffi::zcopy_
    }
}

impl ::Copy for f32 {
    fn copy() -> Fn<f32> {
        ffi::scopy_
    }
}

impl ::Copy for f64 {
    fn copy() -> Fn<f64> {
        ffi::dcopy_
    }
}
