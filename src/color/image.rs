use image::{self, imageops, RgbImage};
use camera::Camera;
use fractal::Fractal;

pub fn edges(image: &RgbImage) -> RgbImage {
    let kernel = [-1.0, -1.0, -1.0,
                  -1.0,  8.0, -1.0,
                  -1.0, -1.0, -1.0];

    imageops::filter3x3(image, &kernel)
}

pub fn produce_image<F, C>(fractal: &F,
                           camera: &Camera,
                           dimensions: (u32, u32),
                           painter: C)
                           -> RgbImage
where
    F: Fractal + ?Sized,
    C: Fn(u8) -> image::Rgb<u8>
{
    let (width, height) = dimensions;
    let mut image = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let coord = (x, y);

            let pos = [x as f64, y as f64];
            let [x, y] = camera.screen_to_world(pos);
            let i = fractal.iterations(x, y);

            image[coord] = painter(i);
        }
    }

    image
}
