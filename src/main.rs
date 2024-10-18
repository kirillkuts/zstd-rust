use std::path::PathBuf;
use std::fs;
use std::io;


use clap::{Parser, Subcommand};
use zstd_rust::frame::{Frame, FrameIterator, SkippableFrame};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// File name to decompress
    file_name: String,

    /// Dump information about frames instead of outputting the result
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    info: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let path = cli.file_name;
    let contents = fs::read(path).unwrap();

    let frame_iterator = FrameIterator::new(&contents);

    for frame in frame_iterator {
        match frame {
            Frame::ZstandardFrame(some) => {
                println!("{:#x?}", some);
            },
            Frame::SkippableFrame(skippableFrame) => {
                println!("{:#x?}", skippableFrame);
            }
        }
    }
}
