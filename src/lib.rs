#![feature(slice_patterns)]

extern crate image;
extern crate palette;
extern crate num_complex;

mod fractal;
mod color;
mod camera;

pub use fractal::Fractal;
pub use fractal::*;
pub use color::{ComplexPalette, SubGradient};
pub use color::*;
pub use camera::Camera;
