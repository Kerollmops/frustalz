extern crate image;
extern crate palette;
extern crate fractalz;

use image::RgbImage;
use palette::Gradient;
use palette::rgb::LinSrgb;

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

    let gradient = Gradient::with_domain(vec![
        (0.0, LinSrgb::new(0.0, 0.027, 0.392)),     // 0, 2.7, 39.2
        (0.16, LinSrgb::new(0.125, 0.42, 0.796)),   // 12.5, 42, 79.6
        (0.42, LinSrgb::new(0.929, 1.0, 1.0)),      // 92.9, 100, 100
        (0.6425, LinSrgb::new(1.0, 0.667, 0.0)),    // 100, 66.7, 0
        (0.8575, LinSrgb::new(0.0, 0.008, 0.0)),    // 0, 0.8, 0
        (1.0, LinSrgb::new(0.0, 0.0, 0.0)),         // 0, 0, 0
    ]);

    for x in 0..width {
        for y in 0..height {
            let coord = (x, y);
            let pos = (x as f64, y as f64);

            let (x, y) = camera.zoom(pos);
            let i = julia.iterations(x, y);

            let color = gradient.get(i as f32 / u8::max_value() as f32);

            image[coord] = image::Rgb { data: color.into_pixel() };
        }
    }

    image.save("./image.png").unwrap();
}
