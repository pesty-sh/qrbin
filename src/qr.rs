use std::io::Read;

use qrcode::QrCode;

use crate::cli::Args;
use crate::error::QrError;

pub fn read_payload(args: &Args) -> Result<Vec<u8>, QrError> {
    let payload = if let Some(value) = &args.payload {
        value.clone().into_bytes()
    } else {
        let mut input = Vec::new();
        std::io::stdin().read_to_end(&mut input)?;
        input
    };

    if payload.is_empty() {
        return Err(QrError::EmptyPayload);
    }

    Ok(payload)
}

pub fn encode(payload: &[u8], ecc: qrcode::EcLevel) -> Result<QrCode, QrError> {
    QrCode::with_error_correction_level(payload, ecc)
        .map_err(|error| QrError::Encoding(error.to_string()))
}
