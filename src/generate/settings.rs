use image::{Antialiazing, ScreenDimensions};
use generate::DateSeed;

#[derive(Debug, Clone, StructOpt)]
pub struct Settings {
    /// The date to use as a seed rounded to the hour,
    /// the default is the current datetime.
    #[structopt(long = "date-seed")]
    pub date_seed: Option<DateSeed>,

    /// Antialiazing used for the images generated (a power of 4).
    #[structopt(long = "antialiazing", default_value = "4")]
    pub antialiazing: Antialiazing,

    /// Screen dimensions used for all image generations.
    #[structopt(long = "screen-dimensions", default_value = "800x600")]
    pub screen_dimensions: Option<ScreenDimensions>,

    /// Whether the program produce all images while diving in the fractal.
    #[structopt(long = "no-debug-images")]
    pub no_debug_images: bool,
}
