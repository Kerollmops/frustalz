mod date_seed;

pub use self::date_seed::DateSeed;

use num_complex::Complex64;
use rand::Rng;
use rand::distributions::{Range, IndependentSample};
use pathfinding::dijkstra;
use image_crate::{imageops, RgbImage, Rgb, FilterType};
use fractal::{Fractal, Mandelbrot, Julia};
use camera::Camera;
use image::{Antialiazing, ComplexPalette, SubGradient, ScreenDimensions};
use image::{produce_image, edges};
use palette::Gradient;
use palette::rgb::LinSrgb;

fn find_point<P>(start: (u32, u32),
                 image: &RgbImage,
                 predicate: P)
                 -> Option<(u32, u32)>
where
    P: Fn(&Rgb<u8>) -> bool
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

#[derive(Debug, Rand)]
pub enum FractalType {
    Mandelbrot,
    Julia,
}

/// Find a good target point that will not be a black area:
///   - create a grayscale image
///   - blur the grayscale image
///   - find the nearest black point
///   - create an edge image of the first grayscaled image
///   - find the nearest white point on the edged image starting from the previous black point
fn find_target_point<F, R>(rng: &mut R,
                           fractal: &F,
                           camera: &Camera,
                           dimensions: (u32, u32))
                           -> Option<(u32, u32)>
where
    F: Fractal + Sync,
    R: Rng
{
    let (width, height) = dimensions;

    let grayscaled = produce_image(fractal, camera, dimensions, |i| Rgb { data: [i; 3] });
    let blurred = imageops::blur(&grayscaled, 10.0);
    let black_point = {
        let start = (rng.gen_range(0, width), rng.gen_range(0, height));
        find_point(start, &blurred, |p| p.data[0] <= 128)
    };

    black_point.and_then(|black_point| {
        let edged = edges(&grayscaled);
        find_point(black_point, &edged, |p| p.data[0] >= 128)
    })
}

fn produce_debug_image<F>(fractal: &F,
                          camera: &Camera,
                          dimensions: (u32, u32),
                          n: usize)
where
    F: Fractal + Sync
{
    let grayscaled = produce_image(fractal, camera, dimensions, |i| Rgb { data: [i; 3] });
    let image = edges(&grayscaled);
    image.save(format!("./spotted-area-{:03}.png", n)).unwrap();
}

#[derive(Debug)]
pub struct Generator<R: Rng> {
    rng: R,
    dive_dimensions: Option<ScreenDimensions>,
    shot_dimensions: Option<ScreenDimensions>,
    antialiazing: Option<Antialiazing>,
    debug_images: bool,
}

impl<R: Rng> Generator<R> {
    pub fn new(rng: R) -> Self {
        Self {
            rng: rng,
            dive_dimensions: None,
            shot_dimensions: None,
            antialiazing: None,
            debug_images: true,
        }
    }

    pub fn dive_dimensions(&mut self, dimensions: ScreenDimensions) -> &mut Self {
        self.dive_dimensions = Some(dimensions);
        self
    }

    pub fn shot_dimensions(&mut self, dimensions: ScreenDimensions) -> &mut Self {
        self.shot_dimensions = Some(dimensions);
        self
    }

    pub fn antialiazing(&mut self, antialiazing: Antialiazing) -> &mut Self {
        self.antialiazing = Some(antialiazing);
        self
    }

    pub fn debug_images(&mut self, debug_images: bool) -> &mut Self {
        self.debug_images = debug_images;
        self
    }

    pub fn generate(mut self) -> RgbImage {
        let dimensions = self.dive_dimensions.unwrap_or(ScreenDimensions(500, 500)).as_tuple();
        let antialiazing: u32 = self.antialiazing.unwrap_or(Antialiazing::new(4).unwrap()).into();

        let (width, height) = dimensions;
        let mut camera = Camera::new([width as f64, height as f64]);

        let (fractal, mut zoom_steps): (Box<Fractal + Sync>, _) = match self.rng.gen() {
            FractalType::Mandelbrot => {
                println!("Mandelbrot");

                let fractal = Mandelbrot::new();
                let zoom_divisions = self.rng.gen_range(3, 40);

                (Box::new(fractal), zoom_divisions)
            },
            FractalType::Julia => {
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

                let sub_gradient = sub_gradients.get(self.rng.gen());
                let gradient = sub_gradient.gradient();
                let ComplexPalette(Complex64 { re, im }) = gradient.get(self.rng.gen());

                println!("Julia ({}, {})", re, im);

                let fractal = Julia::new(re, im);
                let zoom_steps = self.rng.gen_range(0, 40);

                (Box::new(fractal), zoom_steps)
            },
        };

        let zoom_distr = Range::new(0.5, 1.0);

        // to zoom in the fractal:
        //   - find a good target point using the current camera
        //   - zoom using the camera into the current image
        //   - repeat the first step until the max number of iteration is reached
        //     or a target point can't be found
        while let Some((x, y)) = find_target_point(&mut self.rng, &fractal, &camera, dimensions) {
            let zoom = camera.zoom;
            let zoom_multiplier = zoom_distr.ind_sample(&mut self.rng);
            camera.target_on([x as f64, y as f64], zoom * zoom_multiplier); // FIXME handle overflow

            if self.debug_images {
                produce_debug_image(&fractal, &camera, dimensions, zoom_steps);
            }

            zoom_steps -= 1;
            if zoom_steps == 0 { break }
        }


        let gradient = Gradient::with_domain(vec![
            (0.0,    LinSrgb::new(0.0,   0.027, 0.392)), // 0,    2.7,  39.2
            (0.16,   LinSrgb::new(0.125, 0.42,  0.796)), // 12.5, 42,   79.6
            (0.42,   LinSrgb::new(0.929, 1.0,   1.0)),   // 92.9, 100,  100
            (0.6425, LinSrgb::new(1.0,   0.667, 0.0)),   // 100,  66.7, 0
            (0.8575, LinSrgb::new(0.0,   0.008, 0.0)),   // 0,    0.8,  0
            (1.0,    LinSrgb::new(0.0,   0.0,   0.0)),   // 0,    0,    0
        ]);

        let painter = |i| {
            let color = gradient.get(i as f32 / 255.0);
            Rgb { data: color.into_pixel() }
        };

        let dimensions = self.shot_dimensions.unwrap_or(ScreenDimensions(800, 600)).as_tuple();
        let (width, height) = dimensions;

        let aa = antialiazing as f64;

        let (bwidth, bheight) = (width * aa as u32, height * aa as u32);
        camera.screen_size = [bwidth as f64, bheight as f64];

        let image = produce_image(&fractal, &camera, (bwidth, bheight), painter);
        imageops::resize(&image, width, height, FilterType::Triangle)
    }
}
