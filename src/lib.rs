//! Traits that map BLAS accelerated types (e.g. `f32`) to their respective BLAS calls (e.g.
//! `saxpy`)
//!
//! This library provides no safe wrappers around the foreign BLAS functions.
//!
//! If you are looking for a BLAS accelerated linear algebra, check out [linalg].
//!
//! [linalg]: https://github.com/japaric/linalg.rs

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(libc)]

extern crate complex;
extern crate libc;

mod ffi;

pub mod axpy;
pub mod copy;
pub mod dot;
pub mod gemm;
pub mod gemv;
pub mod scal;

/// Transpose matrix before operation?
#[derive(Clone, Copy)]
#[repr(i8)]
pub enum Transpose {
    /// Don't transpose
    No = 110, // 'n'
    /// Transpose
    Yes = 116,  // 't'
}

/// Types with `axpy` acceleration
pub trait Axpy {
    /// Returns the foreign `axpy` function
    fn axpy() -> axpy::Fn<Self>;
}

/// Types with `copy` acceleration
pub trait Copy {
    /// Returns the foreign `copy` function
    fn copy() -> copy::Fn<Self>;
}

/// Types with `dot` acceleration
pub trait Dot {
    /// Returns the foreign `dot` function
    fn dot() -> dot::Fn<Self>;
}

/// Types with `gemm` acceleration
pub trait Gemm {
    /// Returns the foreign `gemm` function
    fn gemm() -> gemm::Fn<Self>;
}

/// Types with `gemv` acceleration
pub trait Gemv {
    /// Returns the foreign `gemv` function
    fn gemv() -> gemv::Fn<Self>;
}

/// Types with `scal` acceleration
pub trait Scal<Alpha=Self> {
    /// Returns the foreign `scal` function
    fn scal() -> scal::Fn<Alpha, Self>;
}

/// The integer used by the BLAS library
// TODO Handle 64-bit BLAS
#[allow(non_camel_case_types)]
pub type blasint = ::libc::c_int;
