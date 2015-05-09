//! nrm2 := ||x||_2

use complex::Complex;

use {blasint, ffi};

/// The signature of `nrm2`
pub type Fn<T, U> = unsafe extern "C" fn (
    *const blasint,
    *const T,
    *const blasint,
) -> U;

impl ::Nrm2 for Complex<f32> {
    type Output = f32;

    fn nrm2() -> Fn<Complex<f32>, f32> {
        ffi::scnrm2_
    }
}

impl ::Nrm2 for Complex<f64> {
    type Output = f64;

    fn nrm2() -> Fn<Complex<f64>, f64> {
        ffi::dznrm2_
    }
}

impl ::Nrm2 for f32 {
    type Output = f32;

    fn nrm2() -> Fn<f32, f32> {
        ffi::snrm2_
    }
}

impl ::Nrm2 for f64 {
    type Output = f64;

    fn nrm2() -> Fn<f64, f64> {
        ffi::dnrm2_
    }
}
