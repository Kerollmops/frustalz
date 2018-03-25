extern crate fractalz;
extern crate structopt;

use structopt::StructOpt;
use fractalz::generate::{generate, Settings};

fn main() {
    let settings = Settings::from_args();

    let image = generate(settings);

    image.save("./image.png").unwrap();
}
