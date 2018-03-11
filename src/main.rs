extern crate image;
extern crate fractalz;

use image::{RgbImage, Rgb};

use fractalz::Fractal;
use fractalz::Julia;

fn main() {
    let julia = Julia::new(0.0, 0.0);

    let (width, height) = (800, 600);
    let mut image = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let coord = (x, y);

            let (x, y) = (x as f64, y as f64);
            let (width, height) = (width as f64, height as f64);

            let i = julia.iterations(x / width, y / height);
            image[coord] = Rgb { data: [i, i, i] };
        }
    }

    image.save("./image.png").unwrap();
}
