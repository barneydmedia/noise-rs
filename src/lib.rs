//! A procedural noise generation library for Rust.
//!
//! # Example
//!
//! ```rust
//! use noise::{NoiseFn, Perlin};
//!
//! let perlin = Perlin::new();
//! let val = perlin.get([42.4, 37.7, 2.8]);
//! ```

#![warn(clippy::all)]
#![deny(missing_copy_implementations)]

#[cfg(feature = "image")]
extern crate image;
extern crate rand;

pub use math::{Point2, Point3, Point4};
pub use noise_fns::*;

mod gradient;
mod math;
mod noise_fns;
mod permutationtable;
pub mod utils;
