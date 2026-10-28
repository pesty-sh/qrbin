use std::io::{self, Write};

use image::Luma;
use qrcode::render::svg;
use qrcode::QrCode;

use crate::cli::{Args, Format};
use crate::error::QrError;

pub fn write(code: &QrCode, args: &Args) -> Result<(), QrError> {
    match args.format {
        Format::Png => write_png(code, args),
        Format::Svg => write_svg(code, args),
    }
}

fn write_png(code: &QrCode, args: &Args) -> Result<(), QrError> {
    let image = code
        .render::<Luma<u8>>()
        .quiet_zone(args.margin > 0)
        .min_dimensions(args.size, args.size)
        .build();

    let mut png = Vec::new();
    image
        .write_to(&mut io::Cursor::new(&mut png), image::ImageFormat::Png)
        .map_err(|error| QrError::Output(error.to_string()))?;

    io::stdout().write_all(&png).map_err(QrError::Io)
}

fn write_svg(code: &QrCode, args: &Args) -> Result<(), QrError> {
    let svg = code
        .render::<svg::Color<'_>>()
        .quiet_zone(args.margin > 0)
        .min_dimensions(args.size, args.size)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();

    writeln!(io::stdout(), "{svg}").map_err(QrError::Io)
}
