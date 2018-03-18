mod julia;
mod mandelbrot;

pub use self::julia::Julia;
pub use self::mandelbrot::Mandelbrot;

pub trait Fractal {
    fn iterations(&self, x: f64, y: f64) -> u8;
}
