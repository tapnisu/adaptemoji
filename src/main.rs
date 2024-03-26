use adaptemoji::cli::Cli;
use clap::{error::ErrorKind, CommandFactory, Parser};

fn main() {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    let mut img = image::open(cli.input)
        .unwrap_or_else(|err| cmd.error(ErrorKind::InvalidValue, err).exit())
        .to_luma_alpha8();

    adaptemoji::convert(&mut img, cli.negative);

    if let Err(err) = img.save(cli.output) {
        cmd.error(ErrorKind::Io, err).exit()
    }
}
