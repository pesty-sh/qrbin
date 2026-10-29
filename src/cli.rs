use std::path::PathBuf;

use clap::Parser;
use serde::Deserialize;

/// Minimal, offline-first QR code generator.
#[derive(Parser, Debug)]
#[command(
    name = "qrbin",
    version,
    about = "Generate QR codes offline. No telemetry. No network access.",
    propagate_version = true
)]
pub struct Args {
    /// Payload to encode. If omitted, read from stdin.
    pub payload: Option<String>,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Png)]
    pub format: Format,

    /// Minimal output width and height in pixels.
    #[arg(short, long, default_value_t = 512)]
    pub size: u32,

    /// Quiet zone. A non-zero value keeps a border; `0` omits it.
    #[arg(short, long, default_value_t = 4)]
    pub margin: usize,

    /// Error-correction level.
    #[arg(short, long, value_enum, default_value_t = Ecc::Medium)]
    pub ecc: Ecc,

    /// Named style. Omit this for the classic renderer.
    #[arg(long, value_enum)]
    pub theme: Option<Theme>,

    /// Local theme file. Command flags override values in the file.
    #[arg(long = "theme-file")]
    pub theme_file: Option<PathBuf>,

    /// Dot and finder color, hex. Requires a theme.
    #[arg(long)]
    pub fg: Option<String>,

    /// Background color, hex. Requires a theme.
    #[arg(long)]
    pub bg: Option<String>,

    /// Gradient on the dots. Use with `--from` and `--to`.
    #[arg(long, value_enum)]
    pub gradient: Option<GradientKind>,

    /// Gradient start color, hex.
    #[arg(long = "from")]
    pub from: Option<String>,

    /// Gradient end color, hex.
    #[arg(long = "to")]
    pub to: Option<String>,

    /// Overall mark shape. Requires a theme.
    #[arg(long, value_enum)]
    pub shape: Option<Shape>,

    /// Local logo file. URLs are rejected.
    #[arg(long)]
    pub logo: Option<String>,

    /// Border width in pixels. Requires a theme. A circle draws this as a ring.
    #[arg(long, visible_alias = "border-width")]
    pub frame: Option<f64>,

    /// Border color, hex.
    #[arg(long = "frame-color", visible_alias = "border-color")]
    pub frame_color: Option<String>,

    /// Text on the border. On a circle it follows the lower ring.
    #[arg(long, visible_alias = "text")]
    pub label: Option<String>,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Plain,
    Rounded,
    Dots,
    Classy,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GradientKind {
    Linear,
    Radial,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Shape {
    Square,
    Circle,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum Format {
    Png,
    Svg,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum Ecc {
    Low,
    Medium,
    Quartile,
    High,
}

impl From<Ecc> for qrcode::EcLevel {
    fn from(value: Ecc) -> Self {
        match value {
            Ecc::Low => qrcode::EcLevel::L,
            Ecc::Medium => qrcode::EcLevel::M,
            Ecc::Quartile => qrcode::EcLevel::Q,
            Ecc::High => qrcode::EcLevel::H,
        }
    }
}
