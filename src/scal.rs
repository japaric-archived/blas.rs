//! x := alpha * x

use complex::Complex;

use {blasint, ffi};

/// The signature of `scal`
pub type Fn<Alpha, T> = unsafe extern "C" fn (
    *const blasint,
    *const Alpha,
    *mut T,
    *const blasint,
);

impl ::Scal for Complex<f32> {
    fn scal() -> Fn<Complex<f32>, Complex<f32>> {
        ffi::cscal_
    }
}

impl ::Scal<f32> for Complex<f32> {
    fn scal() -> Fn<f32, Complex<f32>> {
        ffi::csscal_
    }
}

impl ::Scal for Complex<f64> {
    fn scal() -> Fn<Complex<f64>, Complex<f64>> {
        ffi::zscal_
    }
}

impl ::Scal<f64> for Complex<f64> {
    fn scal() -> Fn<f64, Complex<f64>> {
        ffi::zdscal_
    }
}

impl ::Scal for f32 {
    fn scal() -> Fn<f32, f32> {
        ffi::sscal_
    }
}

impl ::Scal for f64 {
    fn scal() -> Fn<f64, f64> {
        ffi::dscal_
    }
}
