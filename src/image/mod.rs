mod antialiazing;
mod complex_palette;
mod screen_dimensions;
mod sub_gradient;

pub use self::antialiazing::Antialiazing;
pub use self::complex_palette::ComplexPalette;
pub use self::screen_dimensions::ScreenDimensions;
pub use self::sub_gradient::SubGradient;

use image::{imageops, FilterType, Rgb, RgbImage};
use rayon::prelude::*;

use crate::camera::Camera;
use crate::fractal::Fractal;

pub fn edges(image: &RgbImage) -> RgbImage {
    let kernel = [-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];
    imageops::filter3x3(image, &kernel)
}

pub fn produce_image<F, C>(
    fractal: &F,
    camera: &Camera,
    dimensions: (u32, u32),
    antialiazing: Option<u32>,
    painter: C,
) -> RgbImage
where
    F: Fractal + ?Sized + Sync,
    C: Fn(u8) -> Rgb<u8> + Sync + Send,
{
    assert!(antialiazing != Some(0), "antialiazing cannot be equal to zero, prefer 1 instead");

    let (width, height) = dimensions;
    let aa = antialiazing.unwrap_or(1) as f64;
    let (bwidth, bheight) = (width * aa as u32, height * aa as u32);
    let camera = Camera { screen_size: [bwidth as f64, bheight as f64], ..*camera };

    let mut image = RgbImage::new(bwidth, bheight);
    image.par_chunks_mut(3).enumerate().for_each(|(i, p)| {
        let x = i as u32 % bwidth;
        let y = (i as u32 - x) / bwidth;

        let pos = [x as f64, y as f64];
        let [x, y] = camera.screen_to_world(pos);
        let i = fractal.iterations(x, y);

        let data = painter(i).data;
        p.copy_from_slice(&data);
    });

    if antialiazing.is_some() {
        imageops::resize(&image, width, height, FilterType::Triangle)
    } else {
        image
    }
}
