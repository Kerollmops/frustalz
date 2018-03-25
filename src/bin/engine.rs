extern crate fractalz;
extern crate image;
extern crate png;
extern crate structopt;
extern crate egg_mode;
extern crate tokio_core;
extern crate futures;

use std::io::BufWriter;
use png::{Encoder, HasParameters};
use structopt::StructOpt;
use image::RgbImage;
use egg_mode::media::{UploadBuilder, media_types};
use egg_mode::tweet::DraftTweet;
use egg_mode::{Token, KeyPair};
use fractalz::generate::{generate, Settings};
use tokio_core::reactor;

fn image_to_png(image: RgbImage) -> Vec<u8> {
    let (width, height) = image.dimensions();
    let buf = image.into_raw();

    let mut out = BufWriter::new(Vec::new());

    {
        let mut encoder = Encoder::new(&mut out, width, height);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Sixteen);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&buf).unwrap();
    }

    out.into_inner().unwrap()
}

fn main() {
    let settings = Settings::from_args();
    let mut core = reactor::Core::new().unwrap();

    let token = Token::Access {
        consumer: KeyPair::new(include_str!("consumer_key"), include_str!("consumer_secret")),
        access: KeyPair::new(include_str!("access_key"), include_str!("access_secret")),
    };
    let handle = core.handle();

    let image = generate(settings);
    let image = image_to_png(image);

    let builder = UploadBuilder::new(image, media_types::image_png());
    let media_handle = core.run(builder.call(&token, &handle)).unwrap();

    let draft = DraftTweet::new("Hey, check out this!").media_ids(&[media_handle.id]);
    let tweet = core.run(draft.send(&token, &handle)).unwrap();

    println!("{:?}", tweet);
}
