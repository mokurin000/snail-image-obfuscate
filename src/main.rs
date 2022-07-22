#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::path::PathBuf;

use clap::Parser;
use clap::ValueHint;
use klask::Settings;

fn main() {
    pretty_env_logger::init();

    let settings = Settings {
        enable_working_dir: Some("output image will be placed here".into()),
        enable_env: Some("please set RUST_LOG as error, warn, info, debug or trace".into()),
        ..Settings::default()
    };

    klask::run_derived(settings, |matches: Args| {
        let Args {input_file, filename: output_file } = matches;
        snail_image_obfuscate::process(input_file, output_file);
    });
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "image to process, suffix must correct", long, value_hint = ValueHint::FilePath)]
    input_file: PathBuf,
    #[clap(help = "output filename, should end with .png")]
    filename: PathBuf,
}
