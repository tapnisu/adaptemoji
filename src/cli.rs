use clap::Parser;
use std::path::PathBuf;

/// Arguments parser
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Where to input image
    #[clap(short, long)]
    pub input: PathBuf,
    /// Where to output image
    #[clap(short, long)]
    pub output: PathBuf,

    /// Invert image
    #[clap(short, long, action)]
    pub negative: bool,
}
