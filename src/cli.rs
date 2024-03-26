//! Utils for adaptemoji CLI

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

    /// Resize image to 100x100
    #[clap(short, long, action)]
    pub resize: bool,

    /// Invert image for better look on dark backgrounds
    #[clap(short, long, action)]
    pub negative: bool,
}
