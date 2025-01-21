#![warn(missing_docs)]

//! Maths library built with SIMD in mind
//!
//! To learn more about SIMD, please refer to the
//! [portable-simd beginner's guide](https://github.com/rust-lang/portable-simd/blob/master/beginners-guide.md)

#![feature(portable_simd)]

/// A module that defines all vector types of the crate
pub mod vector;

/// A module that exports the maths primitive types
mod types;
pub use types::*;
