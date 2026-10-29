use std::io::{self, Write};

use image::Luma;
use qr_code_styling::rendering::RasterRenderer;
use qr_code_styling::{
    BackgroundOptions, BorderPlugin, CornerDotType, CornerSquareType, CornersDotOptions,
    CornersSquareOptions, DotType, DotsOptions, ErrorCorrectionLevel, Gradient, ImageOptions,
    OutputFormat, Position, QRBorderOptions, QRCodeStyling, QROptions, ShapeType,
};
use qrcode::render::svg;
use qrcode::QrCode;

use crate::cli::{Args, Ecc, Format, GradientKind, Shape, Theme};
use crate::error::QrError;
use crate::theme::ResolvedTheme;

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

pub fn write_themed(payload: &[u8], args: &Args, style: &ResolvedTheme) -> Result<(), QrError> {
    let data = std::str::from_utf8(payload)
        .map_err(|_| QrError::Encoding("payload is not valid text".to_string()))?;
    let (dot, corner_square, corner_dot) = theme_parts(style.theme);
    let mut dots = DotsOptions::new(dot).with_color(style.fg);
    if let Some(gradient) = &style.gradient {
        let paint = match gradient.kind {
            GradientKind::Linear => Gradient::simple_linear(gradient.from, gradient.to),
            GradientKind::Radial => Gradient::simple_radial(gradient.from, gradient.to),
        };
        dots = dots.with_gradient(paint);
    }

    let mut builder = QRCodeStyling::builder()
        .data(data)
        .size(args.size)
        .margin(themed_margin(args, style))
        .shape(match style.shape {
            Shape::Square => ShapeType::Square,
            Shape::Circle => ShapeType::Circle,
        })
        .qr_options(QROptions::new().with_error_correction_level(ecc_level(args.ecc)))
        .dots_options(dots)
        .corners_square_options(CornersSquareOptions::new(corner_square).with_inherited_color())
        .corners_dot_options(CornersDotOptions::new(corner_dot).with_inherited_color())
        .background_options(BackgroundOptions::new(style.bg));
    if let Some(logo) = &style.logo {
        builder = builder.image(logo.clone()).image_options(
            ImageOptions::new()
                .with_hide_background_dots(true)
                .with_image_size(0.25),
        );
    }
    let styled = builder.build().map_err(|error| style_error(&error, data))?;

    let mut svg = styled
        .render_svg()
        .map_err(|error| style_error(&error, data))?;
    if let Some(frame) = &style.frame {
        let mut border = QRBorderOptions::new(frame.thickness, frame.color.to_hex());
        if style.shape == Shape::Circle {
            border = border.with_round(1.0);
        }
        if let Some(label) = &frame.label {
            let font_size = (frame.thickness * 0.42).clamp(14.0, 72.0);
            let label_style = format!(
                "font-size: {font_size:.0}px; font-family: Arial, sans-serif; font-weight: 700; letter-spacing: 0.16em; fill: {};",
                style.bg.to_hex()
            );
            border = border.with_styled_text(Position::Bottom, label, label_style);
        }
        svg = BorderPlugin::new(border).apply(&svg, args.size, args.size);
    }

    let bytes = match args.format {
        Format::Png => RasterRenderer::render(&svg, args.size, args.size, OutputFormat::Png)
            .map_err(|error| style_error(&error, data))?,
        Format::Svg => svg.into_bytes(),
    };
    io::stdout().write_all(&bytes).map_err(QrError::Io)
}

fn theme_parts(theme: Theme) -> (DotType, CornerSquareType, CornerDotType) {
    match theme {
        Theme::Plain => (
            DotType::Square,
            CornerSquareType::Square,
            CornerDotType::Square,
        ),
        Theme::Rounded => (
            DotType::Rounded,
            CornerSquareType::ExtraRounded,
            CornerDotType::Dot,
        ),
        Theme::Dots => (DotType::Dots, CornerSquareType::Dot, CornerDotType::Dot),
        Theme::Classy => (
            DotType::ClassyRounded,
            CornerSquareType::ExtraRounded,
            CornerDotType::Dot,
        ),
    }
}

fn themed_margin(args: &Args, style: &ResolvedTheme) -> u32 {
    let quiet = if args.margin == 0 {
        0
    } else {
        (args.size / 16).max(1)
    };
    let ring_gap = match (&style.frame, style.shape) {
        (Some(frame), Shape::Circle) => (frame.thickness.ceil() as u32).saturating_add(16),
        _ => 0,
    };
    quiet.max(ring_gap)
}

fn style_error(error: &impl std::fmt::Display, payload: &str) -> QrError {
    let message = error.to_string();
    if message.contains(payload) {
        QrError::Output("QR encoding error".to_string())
    } else {
        QrError::Output(message)
    }
}

fn ecc_level(ecc: Ecc) -> ErrorCorrectionLevel {
    match ecc {
        Ecc::Low => ErrorCorrectionLevel::L,
        Ecc::Medium => ErrorCorrectionLevel::M,
        Ecc::Quartile => ErrorCorrectionLevel::Q,
        Ecc::High => ErrorCorrectionLevel::H,
    }
}
