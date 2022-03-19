use std::{collections::BTreeMap, fs::File, io::Write};

use anyhow::Result;

use image::io::Reader as ImageReader;
use poloto::{plotnum::PlotNum, prelude::*, Plotter};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File path to the image
    #[clap(long, default_value = "cat")]
    file: String,
    /// BIN value to use
    #[clap(long, default_value = "1")]
    bin: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file_name = args.file;
    let img = ImageReader::open(format!("images/{file_name}.bmp"))?
        .decode()?
        .grayscale()
        .to_rgba8();

    let total_pixels = {
        let (x, y) = img.dimensions();
        x * y
    };
    let mut pixel_data = Vec::new();
    let mut pmf_data = Vec::new();
    let mut pmf_map = BTreeMap::new();

    for (_, _, color) in img.enumerate_pixels() {
        *pmf_map.entry(bin(args.bin, color[0].into())).or_insert(0) += 1;
        
    }

    for idx in 0..=255 {
        pixel_data.push(*pmf_map.get(&(idx as usize)).unwrap_or(&0));
        pmf_data.push(*pmf_map.get(&(idx as usize)).unwrap_or(&0) as f64 / total_pixels as f64);
    }

    draw_pixel_histogram(pixel_data, &file_name)?;
    draw_pmf_histogram(pmf_data, &file_name)?;

    Ok(())
}

fn bin(bin_val: usize, value: usize) -> usize {
    value / bin_val
}

fn draw_pixel_histogram(data: Vec<i128>, name: &str) -> Result<()> {
    let mut pp = prepare_svg(
        data,
        "pixel".to_string(),
        "Pixel Histogram".to_string(),
        "Brightness Level".to_string(),
        "Occurence Count".to_string(),
    );
    let mut pixel_file = File::create(format!("{name}_pixel_histogram.svg")).unwrap();
    let svg = format!("{}", poloto::disp(|w| pp.simple_theme(w)));
    write!(pixel_file, "{}", &svg).unwrap();

    Ok(())
}
fn draw_pmf_histogram(data: Vec<f64>, name: &str) -> Result<()> {
    let mut pp = prepare_svg(
        data,
        "pmf".to_string(),
        "Probability Mass Function".to_string(),
        "Brightness Level".to_string(),
        "Probability".to_string(),
    );

    let mut pixel_file = File::create(format!("{name}_pmf_histogram.svg")).unwrap();
    let svg = format!("{}", poloto::disp(|w| pp.simple_theme(w)));
    write!(pixel_file, "{}", &svg).unwrap();

    Ok(())
}

fn prepare_svg<T: PlotNum + poloto::plotnum::HasDefaultTicks + 'static>(
    data: Vec<T>,
    histogram_name: String,
    title: String,
    x_axis: String,
    y_axis: String,
) -> Plotter<impl poloto::Disp> {
    let step = data.len() / 10;
    let pmf_data = poloto::data()
        .histogram(histogram_name, (0..).zip(data.into_iter()))
        .build();
    let canvas = poloto::canvas().build();

    let (xtick, xtick_fmt) = poloto::ticks_from_iter((0..).step_by(step));
    let (ytick, ytick_fmt) = poloto::ticks_from_default(pmf_data.boundy(&canvas));
    let pp = pmf_data.stage_with(canvas).plot_with(
        xtick,
        ytick,
        poloto::plot_fmt(
            title,
            x_axis,
            y_axis,
            xtick_fmt.with_tick_fmt(|w, v| write!(w, "{}", v)),
            ytick_fmt,
        ),
    );
    pp
}
