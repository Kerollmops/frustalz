use crate::fractal::Fractal;
use num_complex::Complex64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Mandelbrot;

impl Mandelbrot {
    pub fn new() -> Mandelbrot {
        Mandelbrot
    }
}

impl Fractal for Mandelbrot {
    fn iterations(&self, x: f64, y: f64) -> u8 {
        let mut iterations = 0;
        let c = Complex64::new(x, y);
        let mut z = c;

        while (z * z).re <= 4.0 && iterations < u8::max_value() {
            z = z * z + c;
            iterations += 1;
        }

        iterations
    }
}
