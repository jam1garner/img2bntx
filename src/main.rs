use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    image: PathBuf,
    out: PathBuf,

    #[structopt(long)]
    width: Option<u32>,

    #[structopt(long)]
    height: Option<u32>,

    #[structopt(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::from_args();

    let image = match (args.width, args.height) {
        (Some(width), Some(height)) => image::DynamicImage::ImageRgba8(image::RgbaImage::new(width, height)),
        _ => image::open(args.image).unwrap()
    };

    bntx::BntxFile::from_image(image, args.name.as_deref().unwrap_or("file"))
        .save(args.out)
        .unwrap()
}
