use num_complex::Complex64;
use palette::Mix;

#[derive(Debug, Copy, Clone)]
pub struct ComplexPalette(pub Complex64);

impl ComplexPalette {
    pub fn new(re: f64, im: f64) -> Self {
        ComplexPalette(Complex64::new(re, im))
    }
}

impl Mix for ComplexPalette {
    type Scalar = f64;

    fn mix(&self, other: &Self, factor: Self::Scalar) -> Self {
        let re = self.0.re + factor * (other.0.re - self.0.re);
        let im = self.0.im + factor * (other.0.im - self.0.im);
        ComplexPalette(Complex64::new(re, im))
    }
}
