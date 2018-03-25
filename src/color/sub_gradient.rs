use color::complex_palette::ComplexPalette;
use palette::Gradient;
use palette::Mix;

#[derive(Debug, Copy, Clone)]
pub struct SubGradient {
    from: ComplexPalette,
    to: ComplexPalette,
}

impl SubGradient {
    pub fn new(from: ComplexPalette, to: ComplexPalette) -> Self {
        SubGradient { from, to }
    }

    pub fn gradient(&self) -> Gradient<ComplexPalette> {
        Gradient::new(vec![self.from, self.to])
    }
}

impl Mix for SubGradient {
    type Scalar = f64;

    fn mix(&self, other: &Self, factor: Self::Scalar) -> Self {
        let from = self.from.mix(&other.from, factor);
        let to = self.to.mix(&other.to, factor);
        SubGradient::new(from, to)
    }
}
