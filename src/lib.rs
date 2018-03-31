extern crate num_complex;
extern crate palette;
extern crate rand;
#[macro_use] extern crate rand_derive;
extern crate pathfinding;
extern crate rayon;
extern crate chrono;
extern crate image as image_crate;
extern crate structopt;

pub mod fractal;
pub mod generate;
pub mod image;
pub mod camera;

pub use fractal::Fractal;
pub use fractal::{Mandelbrot, Julia};
pub use camera::Camera;
