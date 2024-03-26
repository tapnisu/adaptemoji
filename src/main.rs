use adaptemoji::cli::Cli;
use clap::{error::ErrorKind, CommandFactory, Parser};

fn main() {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    let img =
        image::open(cli.input).unwrap_or_else(|err| cmd.error(ErrorKind::InvalidValue, err).exit());

    let mut gray_img = if cli.resize {
        img.resize_to_fill(100, 100, image::imageops::FilterType::Triangle)
    } else {
        img
    }
    .to_luma_alpha8();

    adaptemoji::convert_adaptive(&mut gray_img, cli.negative);

    if let Err(err) = gray_img.save(cli.output) {
        cmd.error(ErrorKind::Io, err).exit()
    }
}
