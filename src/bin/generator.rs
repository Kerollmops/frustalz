extern crate fractalz;
extern crate rand;
#[macro_use] extern crate structopt;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use rand::{StdRng, SeedableRng};
use structopt::StructOpt;
use fractalz::{
    generate::{Generator, DateSeed},
    image::{Antialiazing, ScreenDimensions},
};

#[derive(Debug, Clone, StructOpt)]
pub struct Settings {
    /// The date to use as a seed rounded to the hour,
    /// the default is the current datetime.
    #[structopt(long = "date-seed")]
    pub date_seed: Option<DateSeed>,

    /// Antialiazing used for the images generated (a power of 4).
    #[structopt(long = "antialiazing")]
    pub antialiazing: Option<Antialiazing>,

    /// Screen dimensions used for all image generations.
    #[structopt(long = "screen-dimensions")]
    pub screen_dimensions: Option<ScreenDimensions>,

    /// Whether the program produce all images while diving in the fractal.
    #[structopt(long = "no-debug-images")]
    pub no_debug_images: bool,
}

fn main() {
    let settings = Settings::from_args();

    let rng = {
        let datetime = settings.date_seed.unwrap_or_default();
        println!("{:?}", datetime);

        let mut s = DefaultHasher::new();
        datetime.hash(&mut s);

        let hash = s.finish();
        StdRng::from_seed(&[hash as usize])
    };

    let mut generator = Generator::new(rng);

    if let Some(dims) = settings.screen_dimensions {
        generator.dimensions(dims);
    }
    if let Some(anti) = settings.antialiazing {
        generator.antialiazing(anti);
    }
    generator.debug_images(!settings.no_debug_images);

    let image = generator.generate();

    match image.save("./image.png") {
        Ok(_) => println!("image saved to \"./image.png\""),
        Err(e) => eprintln!("can not save image to \"./image.png\": {}", e),
    }
}
