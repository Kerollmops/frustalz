#![feature(slice_patterns)]

extern crate num_complex;
extern crate image as image_crate;
extern crate palette;
extern crate rand;
#[macro_use] extern crate rand_derive;
extern crate pathfinding;
#[macro_use] extern crate structopt;
extern crate chrono;

pub mod fractal;
pub mod generate;
pub mod image;
pub mod camera;

pub use fractal::Fractal;
pub use fractal::{Mandelbrot, Julia};
pub use camera::Camera;
