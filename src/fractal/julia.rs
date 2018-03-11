use num_complex::Complex64;
use fractal::Fractal;

pub struct Julia {
    x: f64,
    y: f64,
    c: Complex64,
}

impl Julia {

    pub fn new(x: f64, y: f64) -> Julia {
        let c = Complex64::new(0.3 + x, 0.5 + y);
        Julia { x, y, c }
    }
}

impl Fractal for Julia {

    fn iterations(&self, x: f64, y: f64) -> u8 {
        let mut iterations = 0;
        let mut z = Complex64::new(x + self.x, y + self.y);

        while (z + z).re <= 4.0 && iterations < 255 {
            z = z * z + self.c;
            iterations += 1;
        }

        // dbl_perc = (double)it / (double)(e->fractal.max_it + 1);
        // get_gradient_color(e->fractal.gradients->content, &dbl_perc, col);

        iterations
    }
}
