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
        let complex = self.0 + factor * (other.0 - self.0);
        ComplexPalette(complex)
    }
}
