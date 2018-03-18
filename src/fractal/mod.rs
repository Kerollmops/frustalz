mod julia;
mod mandelbrot;

use std::ops::Deref;

pub use self::julia::Julia;
pub use self::mandelbrot::Mandelbrot;

pub trait Fractal {
    fn iterations(&self, x: f64, y: f64) -> u8;
}

impl<T: Fractal + ?Sized> Fractal for Box<T> {
    fn iterations(&self, x: f64, y: f64) -> u8 {
        self.deref().iterations(x, y)
    }
}
