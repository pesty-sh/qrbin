use clap::Parser;

/// Minimal, offline-first QR code generator.
#[derive(Parser, Debug)]
#[command(
    name = "qrbin",
    version,
    about = "Generate QR codes offline. No telemetry. No network access.",
    propagate_version = true
)]
pub struct Args {
    /// Payload to encode, If omitted, read from stdin
    pub payload: Option<String>,

    /// Output format.
    #[arg(short, long, value_enum, default_value_t = Format::Png)]
    pub format: Format,

    /// Minimal output width and height in pixels
    #[arg(short, long, default_value_t = 512)]
    pub size: u32,

    /// Quiet zone. A non-zero value keeps the standard 4-module border; 0 omits it.
    #[arg(short, long, default_value_t = 4)]
    pub margin: usize,

    /// Error-correction level
    #[arg(short, long, value_enum, default_value_t = Ecc::Medium)]
    pub ecc: Ecc,
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
