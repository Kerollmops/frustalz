#![feature(slice_patterns)]

extern crate image;
extern crate palette;
extern crate rand;
extern crate pathfinding;
extern crate fractalz;

use image::FilterType;
use image::RgbImage;
use image::imageops;
use palette::Gradient;
use palette::rgb::LinSrgb;
use rand::{SeedableRng, Rng};
use rand::StdRng;
use pathfinding::dijkstra;

use fractalz::Fractal;
use fractalz::{Julia, Mandelbrot};
use fractalz::Camera;

fn edges(image: &RgbImage) -> RgbImage {
    let kernel = [-1.0, -1.0, -1.0,
                  -1.0,  8.0, -1.0,
                  -1.0, -1.0, -1.0];

    imageops::filter3x3(image, &kernel)
}

fn produce_image<F, C>(fractal: &F,
                       camera: &Camera,
                       dimensions: (u32, u32),
                       coloriser: C)
                       -> RgbImage
where
    F: Fractal,
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

            image[coord] = coloriser(i);
        }
    }

    image
}

fn find_point<P>(start: (u32, u32),
                 image: &RgbImage,
                 predicate: P)
                 -> Option<(u32, u32)>
where
    P: Fn(&image::Rgb<u8>) -> bool
{
    let (width, height) = image.dimensions();

    let result = dijkstra(&start, |&(x, y)| {
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push(((x - 1, y), 1))
        }
        if y > 0 {
            neighbours.push(((x, y - 1), 1))
        }
        if x < width - 1 {
            neighbours.push(((x + 1, y), 1))
        }
        if y < height - 1 {
            neighbours.push(((x, y + 1), 1))
        }
        neighbours
    },
    |&(x, y)| predicate(&image.get_pixel(x, y)));

    result.map(|(path, _)| *path.last().unwrap())
}

fn main() {
    // let mut rng = StdRng::from_seed(&[42, 42]);
    let mut rng = StdRng::new().unwrap();
    // let fractal = Julia::new(0.0, 0.0);
    let fractal = Mandelbrot::new();

    let dimensions = (800, 600);
    let (width, height) = dimensions;

    let mut camera = Camera::new([width as f64, height as f64]);

    // to find a good target point that will not be a black area:
    // - create a grayscale image
    // - blur the grayscale image
    // - find the nearest black point
    // - create an edge image of the first grayscaled image
    // - find the nearest white point on the edged image starting from the previous black point
    let target_point = {
        let grayscaled = produce_image(&fractal, &camera, dimensions, |i| image::Rgb { data: [i; 3] });
        let blurred = imageops::blur(&grayscaled, 10.0);
        let black_point = {
            let start = (rng.gen_range(0, width), rng.gen_range(0, height));
            find_point(start, &blurred, |p| p.data[0] <= 128).expect("no black point found")
        };
        let edged = edges(&grayscaled);
        let white_point = find_point(black_point, &edged, |p| p.data[0] >= 128);

        white_point
    };

    let gradient = Gradient::with_domain(vec![
        (0.0, LinSrgb::new(0.0, 0.027, 0.392)),     // 0, 2.7, 39.2
        (0.16, LinSrgb::new(0.125, 0.42, 0.796)),   // 12.5, 42, 79.6
        (0.42, LinSrgb::new(0.929, 1.0, 1.0)),      // 92.9, 100, 100
        (0.6425, LinSrgb::new(1.0, 0.667, 0.0)),    // 100, 66.7, 0
        (0.8575, LinSrgb::new(0.0, 0.008, 0.0)),    // 0, 0.8, 0
        (1.0, LinSrgb::new(0.0, 0.0, 0.0)),         // 0, 0, 0
    ]);

    let colorizer = |i| {
        let color = gradient.get(i as f32 / 255.0);
        image::Rgb { data: color.into_pixel() }
    };

    // antialiazing (power of 2)
    let aa = 1.0;

    let zoom = rng.gen_range(10e-8, 10e-4);

    if let Some((x, y)) = target_point {
        let center = camera.screen_to_world([x as f64, y as f64]);
        println!("position: {:?}", center);
        println!("zoom: {:?}", zoom);
    }

    // once the targeted point has been found
    // - zoom to the target spot
    // - create a colorful image of this spot
    let (bwidth, bheight) = (width * aa as u32, height * aa as u32);
    camera.screen_size = [bwidth as f64, bheight as f64];
    let (x, y) = target_point.expect("no starting point found");
    camera.target_on([x as f64 * aa, y as f64 * aa], zoom);
    let image = produce_image(&fractal, &camera, (bwidth, bheight), colorizer);
    let image = imageops::resize(&image, width, height, FilterType::Triangle);


    image.save("./image.png").unwrap();
}
