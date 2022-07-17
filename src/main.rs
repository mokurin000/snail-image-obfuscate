#![feature(generators, generator_trait)]
#![allow(unused_variables)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use clap::{App, Arg};
use image::ImageFormat;

use image::io::Reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    // return indices in "snail sort", for example,
    // (3,3) will get [(0,0),(0,1),(0,2),(1,2),(2,2),(2,1),(2,0),(1,0),(1,1)]
    let mut snail_sort = |(height_, width_): (u32, u32)| {
        fn is_odd(num: u32) -> u32 {
            2 - (num & 1)
        }

        for ((width, height), off) in (is_odd(width_)..=width_)
            .rev()
            .step_by(2)
            .zip((is_odd(height_)..=height_).rev().step_by(2))
            .zip(0u32..)
        {
            if width == 1 {
                for (h, w) in (0..height).map(move |i| (off + i, off)) {
                    yield (h, w);
                }
                return;
            }

            if height == 1 {
                for (h, w) in (0..width).map(move |i| (off, off + i)) {
                    yield (h, w);
                }
                return;
            }

            for (h, w) in (0..width - 1).map(move |i| (off, i + off)) {
                yield (h, w);
            }
            for (h, w) in (0..height - 1).map(move |i| (off + i, off + width - 1)) {
                yield (h, w);
            }
            for (h, w) in (1..width).rev().map(move |i| (off + height - 1, off + i)) {
                yield (h, w);
            }
            for (h, w) in (1..height).rev().map(move |i| (i + off, off)) {
                yield (h, w);
            }
        }
    };

    let mut snail_sort_rev = |(height_, width_): (u32, u32)| {
        fn offset_end(num1: u32, num2: u32) -> u32 {
            (num1.min(num2) - 1) / 2
        }

        let offset = offset_end(width_, height_);
        for ((width, height), off) in (width_ - offset * 2..=width_)
            .step_by(2)
            .zip((height_ - offset * 2..=height_).step_by(2))
            .zip((0..=offset).rev())
        {
            if width == 1 {
                for (h, w) in (0..height).map(move |i| (off + i, off)).rev() {
                    yield (h, w);
                }
                continue;
            } else if height == 1 {
                for (h, w) in (0..width).map(move |i| (off, off + i)).rev() {
                    yield (h, w);
                }
                continue;
            }

            for (h, w) in (1..height).map(move |i| (i + off, off)) {
                yield (h, w);
            }

            for (h, w) in (1..width).map(move |i| (off + height - 1, off + i)) {
                yield (h, w);
            }

            for (h, w) in (0..height - 1)
                .map(move |i| (off + i, off + width - 1))
                .rev()
            {
                yield (h, w);
            }
            for (h, w) in (0..width - 1).map(move |i| (off, i + off)).rev() {
                yield (h, w);
            }
        }
    };

    let matches = App::new("Snail Image Confuse")
        .author("poly000 <1348292515@qq.com>")
        .about("Simple image confuse tool")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file to write (*.png)")
                .required(true)
                .index(2),
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();

    info!("INPUT: {}", input);
    info!("OUTPUT: {}", output);

    let input_img = Reader::open(input)?.decode()?;
    let writer = std::io::BufWriter::new(std::fs::File::create(output)?);

    let height = input_img.height();
    let width = input_img.width();
    info!("height & width: {}, {}", height, width);

    let pixels = height * width / 2;
    debug!("pixels: {}", pixels);
    info!("start converting input image to RGBA16");
    let mut mut_img = input_img.to_rgba16();
    for _ in 0..pixels {
        if let GeneratorState::Yielded((hy, hx)) = Pin::new(&mut snail_sort).resume((height, width))
        {
            if let GeneratorState::Yielded((ty, tx)) =
                Pin::new(&mut snail_sort_rev).resume((height, width))
            {
                let p = *mut_img.get_pixel(hx, hy);
                *mut_img.get_pixel_mut(hx, hy) = *mut_img.get_pixel(tx, ty);
                *mut_img.get_pixel_mut(tx, ty) = p;
            }
        }
    }

    info!("start writing to {}", output);
    mut_img.save_with_format(output, ImageFormat::Png)?;
    Ok(())
}
