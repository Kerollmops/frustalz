extern crate image;
extern crate fractalz;

use image::{RgbImage, Rgb};

use fractalz::Fractal;
use fractalz::Julia;
use fractalz::CameraZoom;

fn main() {
    let julia = Julia::new(0.0, 0.0);

    let (width, height) = (800, 600);
    let mut image = RgbImage::new(width, height);

    let camera = CameraZoom {
        dimensions: (width as f64, height as f64),
        zoom: 1.0,
        to: (0.0, 0.0),
    };

    for x in 0..width {
        for y in 0..height {
            let coord = (x, y);
            let pos = (x as f64, y as f64);

            let (x, y) = camera.zoom(pos);
            let i = julia.iterations(x, y);

            image[coord] = Rgb { data: [i, i, i] };
        }
    }

    image.save("./image.png").unwrap();
}
