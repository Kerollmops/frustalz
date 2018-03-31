mod antialiazing;
mod complex_palette;
mod sub_gradient;
mod screen_dimensions;

pub use self::antialiazing::Antialiazing;
pub use self::complex_palette::ComplexPalette;
pub use self::sub_gradient::SubGradient;
pub use self::screen_dimensions::ScreenDimensions;

use image_crate::{imageops, RgbImage, Rgb};
use rayon::prelude::*;
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
    F: Fractal + ?Sized + Sync,
    C: Fn(u8) -> Rgb<u8> + Sync + Send
{
    let (width, height) = dimensions;
    let mut image = RgbImage::new(width, height);

    image.par_chunks_mut(3)
        .enumerate()
        .for_each(|(i, p)| {
            let x = i as u32 % width;
            let y = (i as u32 - x) / width;

            let pos = [x as f64, y as f64];
            let pos = camera.screen_to_world(pos);
            let (x, y) = (pos[0], pos[1]);
            let i = fractal.iterations(x, y);

            let data = painter(i).data;
            p.copy_from_slice(&data);
        });

    image
}
