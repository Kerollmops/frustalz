use std::fmt;
use num_complex::Complex64;
use generate::FractalType;

#[derive(Debug)]
pub struct FractalInfo {
    pub fractal_type: FractalType,
    pub domain: Complex64,
    pub position: [f64; 2],
    pub zoom: f64,
}

impl fmt::Display for FractalInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y) = (self.position[0], self.position[1]);

        match self.fractal_type {
            FractalType::Julia => {
                let Complex64 { re, im } = self.domain;
                write!(f, "Here is the julia fractal in the domain ({:.3}, {:.3}), \
                       focus is on the area ({:.3}, {:.3}) with the zoom set on {:.3}x.",
                        re, im, x, y, self.zoom.recip())
            },
            FractalType::Mandelbrot => {
                write!(f, "Here is the mandelbrot fractal, \
                       focus is on the area ({:.3}, {:.3}) with the zoom set on {:.3}x.",
                        x, y, self.zoom.recip())
            },
        }
    }
}
