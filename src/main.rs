#![feature(slice_patterns)]

extern crate image;
extern crate palette;
extern crate rand;
extern crate pathfinding;
extern crate fractalz;

use image::GenericImage;
use image::RgbImage;
use image::imageops::filter3x3;
use palette::Gradient;
use palette::rgb::LinSrgb;
use rand::{SeedableRng, Rng};
use rand::StdRng;
use pathfinding::dijkstra;

use fractalz::Fractal;
use fractalz::Julia;
use fractalz::Camera;

fn grayscale<F: Fractal>(fractal: &F,
                         dimensions: (u32, u32),
                         camera: &Camera)
                         -> RgbImage
{
    let (width, height) = dimensions;
    let mut image = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let coord = (x, y);

            let pos = [x as f64, y as f64];
            let [x, y] = camera.screen_to_world(pos);
            let i = fractal.iterations(x, y);

            image[coord] = image::Rgb { data: [i; 3] };
        }
    }

    image
}

fn edges<F: Fractal>(fractal: &F,
                     dimensions: (u32, u32),
                     camera: &Camera)
                     -> RgbImage
{
    let grayscale = grayscale(fractal, dimensions, camera);
    let kernel = [-1.0, -1.0, -1.0,
                  -1.0,  8.0, -1.0,
                  -1.0, -1.0, -1.0];

    filter3x3(&grayscale, &kernel)
}

fn nearest_interresting_point(edges: &RgbImage) -> Option<(u32, u32)> {
    let mut rng = StdRng::from_seed(&[42, 42, 42]);
    let x = rng.gen_range(0, edges.width());
    let y = rng.gen_range(0, edges.height());

    let start = (x, y);
    let result = dijkstra(&start, |&(x, y)| {
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push(((x - 1, y), 1))
        }
        if y > 0 {
            neighbours.push(((x, y - 1), 1))
        }
        if x < edges.width() - 1 {
            neighbours.push(((x + 1, y), 1))
        }
        if y < edges.height() - 1 {
            neighbours.push(((x, y + 1), 1))
        }
        neighbours
    },
    |&(x, y)| edges.get_pixel(x, y).data[0] >= u8::max_value() / 2);

    result.map(|(path, _)| *path.last().unwrap())
}

fn colorfull<F: Fractal>(fractal: &F,
                         dimensions: (u32, u32),
                         camera: Camera)
                         -> RgbImage
{
    let gradient = Gradient::with_domain(vec![
        (0.0, LinSrgb::new(0.0, 0.027, 0.392)),     // 0, 2.7, 39.2
        (0.16, LinSrgb::new(0.125, 0.42, 0.796)),   // 12.5, 42, 79.6
        (0.42, LinSrgb::new(0.929, 1.0, 1.0)),      // 92.9, 100, 100
        (0.6425, LinSrgb::new(1.0, 0.667, 0.0)),    // 100, 66.7, 0
        (0.8575, LinSrgb::new(0.0, 0.008, 0.0)),    // 0, 0.8, 0
        (1.0, LinSrgb::new(0.0, 0.0, 0.0)),         // 0, 0, 0
    ]);

    let (width, height) = dimensions;
    let mut image = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let coord = (x, y);
            let pos = [x as f64, y as f64];

            let [x, y] = camera.screen_to_world(pos);
            let i = fractal.iterations(x, y);

            let color = gradient.get(i as f32 / u8::max_value() as f32);

            image[coord] = image::Rgb { data: color.into_pixel() };
        }
    }

    image
}

fn main() {
    let julia = Julia::new(0.0, 0.0);

    let dimensions = (800, 600);
    let (width, height) = dimensions;

    let camera = Camera::new([width as f64, height as f64]);

    let image = edges(&julia, dimensions, &camera);
    let (x, y) = nearest_interresting_point(&image).unwrap();

    // create debug subimage
    {
        let mut image = edges(&julia, dimensions, &camera);
        {
            let mut subimage = image.sub_image(x - 3, y - 3, 6, 6);
            for (_, _, p) in subimage.pixels_mut() {
                *p = image::Rgb { data: [255, 0, 0] };
            }
        }
        image.save("./spotted-area.png").unwrap();
    }

    let mut camera = Camera::new([width as f64, height as f64]);
    camera.target_on([x as f64, y as f64], 1.0);

    let mut image = colorfull(&julia, dimensions, camera);

    image.save("./image.png").unwrap();
}
