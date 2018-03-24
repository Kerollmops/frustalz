#![feature(slice_patterns)]

extern crate num_complex;
extern crate image;
extern crate palette;
extern crate rand;
#[macro_use] extern crate rand_derive;
extern crate pathfinding;
extern crate chrono;
#[macro_use] extern crate structopt;
extern crate fractalz;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use num_complex::Complex64;
use image::FilterType;
use image::RgbImage;
use image::imageops;
use palette::Mix;
use palette::Gradient;
use palette::rgb::LinSrgb;
use rand::{SeedableRng, Rng};
use rand::StdRng;
use pathfinding::dijkstra;
use structopt::StructOpt;
use chrono::{Utc, DateTime, Timelike};

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

fn floor_to_hour(datetime: DateTime<Utc>) -> DateTime<Utc> {
    datetime
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap()
}

#[derive(Debug, StructOpt)]
struct Settings {
    /// The date to use as a seed
    #[structopt(long = "date-seed")]
    date_seed: Option<DateTime<Utc>>,

    /// Antialiazing used for the images generated (power of 4).
    #[structopt(long = "antialiazing", default_value = "4")]
    antialiazing: u32,
}

#[derive(Debug, Rand)]
enum RandFractal {
    Mandelbrot,
    Julia,
}

#[derive(Debug, Copy, Clone)]
struct ComplexPalette(Complex64);

impl ComplexPalette {
    fn new(re: f64, im: f64) -> Self {
        ComplexPalette(Complex64::new(re, im))
    }
}

impl Mix for ComplexPalette {
    type Scalar = f64;

    fn mix(&self, other: &Self, factor: Self::Scalar) -> Self {
        let complex = self.0 + factor * (other.0 - self.0);
        ComplexPalette(complex)
    }
}

#[derive(Debug, Copy, Clone)]
struct SubGradient {
    from: ComplexPalette,
    to: ComplexPalette,
}

impl SubGradient {
    fn new(from: ComplexPalette, to: ComplexPalette) -> Self {
        SubGradient { from, to }
    }

    fn gradient(&self) -> Gradient<ComplexPalette> {
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

fn main() {
    let settings = Settings::from_args();

    let mut rng = {
        let datetime = settings.date_seed.unwrap_or(Utc::now());
        let datetime = floor_to_hour(datetime);

        println!("{:?}", datetime);

        let mut s = DefaultHasher::new();
        datetime.hash(&mut s);
        let hash = s.finish();

        StdRng::from_seed(&[hash as usize])
    };

    let dimensions = (800, 600);
    let (width, height) = dimensions;
    let mut camera = Camera::new([width as f64, height as f64]);

    let (fractal, zoom): (Box<Fractal>, _) = match rng.gen() {
        RandFractal::Mandelbrot => {
            let zoom = rng.gen_range(10e-7, 10e-4);
            let mandelbrot = Mandelbrot::new();

            println!("Mandelbrot");

            (Box::new(mandelbrot), zoom)
        },
        RandFractal::Julia => {
            // https://upload.wikimedia.org/wikipedia/commons/a/a9/Julia-Teppich.png
            let sub_gradients = Gradient::new(vec![
                SubGradient::new(ComplexPalette::new(-0.8,  0.4), ComplexPalette::new(-0.8,  0.0)),
                SubGradient::new(ComplexPalette::new(-0.6,  0.8), ComplexPalette::new(-0.6,  0.6)),
                SubGradient::new(ComplexPalette::new(-0.4,  0.8), ComplexPalette::new(-0.4,  0.6)),
                SubGradient::new(ComplexPalette::new(-0.2,  1.0), ComplexPalette::new(-0.2,  0.8)),
                SubGradient::new(ComplexPalette::new( 0.0,  1.0), ComplexPalette::new( 0.0,  0.8)),
                SubGradient::new(ComplexPalette::new( 0.19, 0.8), ComplexPalette::new( 0.19, 0.6)),
                SubGradient::new(ComplexPalette::new( 0.49, 0.6), ComplexPalette::new( 0.49, 0.2)),
            ]);

            let i = rng.gen();
            let sub_gradient = sub_gradients.get(i);

            let gradient = sub_gradient.gradient();
            let i = rng.gen();
            let ComplexPalette(Complex64 { re, im }) = gradient.get(i);

            let zoom = 1.0;

            println!("for i = {:?}", i);
            println!("Julia ({}, {})", re, im);

            let julia = Julia::new(re, im);

            (Box::new(julia), zoom)
        },
    };

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
            find_point(start, &blurred, |p| p.data[0] <= 128)
        };
        black_point.and_then(|black_point| {
            let edged = edges(&grayscaled);
            find_point(black_point, &edged, |p| p.data[0] >= 128)
        })
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

    if let Some((x, y)) = target_point {
        let center = camera.screen_to_world([x as f64, y as f64]);
        println!("position: {:?}", center);
        println!("zoom: {:?}", zoom);
    }

    // create debug subimage
    {
        let grayscaled = produce_image(&fractal, &camera, dimensions, |i| image::Rgb { data: [i; 3] });
        let image = edges(&grayscaled);
        image.save("./spotted-area.png").unwrap();
    }

    let aa = settings.antialiazing as f64;

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
