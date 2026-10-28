//! Minimal, offline-first QR code generator.
mod cli;
mod error;
mod output;
mod qr;

use clap::Parser;
use cli::Args;

// V1
// fn main() {
//     println!("Hello, world!");
// }

// V2
fn main() {
    let args = Args::parse();

    if let Err(error) = run(&args) {
        eprintln!("qrbin error: {error}");
        std::process::exit(1);
    }
}

fn run(args: &Args) -> Result<(), error::QrError> {
    let payload = qr::read_payload(args)?;
    let code = qr::encode(&payload, args.ecc.into())?;
    output::write(&code, args)?;

    Ok(())
}
