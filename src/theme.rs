use std::fs;
use std::path::Path;

use qr_code_styling::Color;
use serde::Deserialize;

use crate::cli::{Args, GradientKind, Shape, Theme};
use crate::error::QrError;

pub struct ResolvedTheme {
    pub theme: Theme,
    pub fg: Color,
    pub bg: Color,
    pub gradient: Option<ResolvedGradient>,
    pub shape: Shape,
    pub logo: Option<Vec<u8>>,
    pub frame: Option<ResolvedFrame>,
}

pub struct ResolvedGradient {
    pub kind: GradientKind,
    pub from: Color,
    pub to: Color,
}

pub struct ResolvedFrame {
    pub thickness: f64,
    pub color: Color,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ThemeFile {
    theme: Option<Theme>,
    fg: Option<String>,
    bg: Option<String>,
    gradient: Option<GradientKind>,
    from: Option<String>,
    to: Option<String>,
    shape: Option<Shape>,
    logo: Option<String>,
    frame: Option<f64>,
    frame_color: Option<String>,
    label: Option<String>,
}

/// `None` means the classic renderer. Style flags without a theme are an error.
pub fn resolve(args: &Args) -> Result<Option<ResolvedTheme>, QrError> {
    let file = match &args.theme_file {
        Some(path) => Some(load_theme_file(path)?),
        None => None,
    };
    let theme = args
        .theme
        .or_else(|| file.as_ref().and_then(|item| item.theme));
    let Some(theme) = theme else {
        if file.is_some() || style_flags_set(args) {
            return Err(QrError::Output(
                "pass --theme, or set theme in --theme-file".to_string(),
            ));
        }
        return Ok(None);
    };

    let fg = parse_color(&pick(
        args.fg.as_deref(),
        file.as_ref().and_then(|item| item.fg.as_deref()),
        "#000000",
    ))?;
    let bg = parse_color(&pick(
        args.bg.as_deref(),
        file.as_ref().and_then(|item| item.bg.as_deref()),
        "#ffffff",
    ))?;
    let gradient = resolve_gradient(args, file.as_ref())?;
    let shape = args
        .shape
        .or_else(|| file.as_ref().and_then(|item| item.shape))
        .unwrap_or(Shape::Square);
    let logo_path = args
        .logo
        .clone()
        .or_else(|| file.as_ref().and_then(|item| item.logo.clone()));
    let logo = match logo_path {
        Some(path) => Some(read_logo(&path)?),
        None => None,
    };
    let frame = resolve_frame(args, file.as_ref())?;

    Ok(Some(ResolvedTheme {
        theme,
        fg,
        bg,
        gradient,
        shape,
        logo,
        frame,
    }))
}

fn style_flags_set(args: &Args) -> bool {
    args.fg.is_some()
        || args.bg.is_some()
        || args.gradient.is_some()
        || args.from.is_some()
        || args.to.is_some()
        || args.shape.is_some()
        || args.logo.is_some()
        || args.frame.is_some()
        || args.frame_color.is_some()
        || args.label.is_some()
}

fn load_theme_file(path: &Path) -> Result<ThemeFile, QrError> {
    let text = fs::read_to_string(path)?;
    toml::from_str(&text).map_err(|error| QrError::Output(format!("invalid theme file: {error}")))
}

fn pick(cli: Option<&str>, file: Option<&str>, default: &str) -> String {
    cli.or(file).unwrap_or(default).to_string()
}

fn parse_color(value: &str) -> Result<Color, QrError> {
    Color::from_hex(value).map_err(|_| QrError::Output(format!("invalid color: {value}")))
}

fn resolve_gradient(
    args: &Args,
    file: Option<&ThemeFile>,
) -> Result<Option<ResolvedGradient>, QrError> {
    let kind = args
        .gradient
        .or_else(|| file.and_then(|item| item.gradient));
    let from = args
        .from
        .as_deref()
        .or_else(|| file.and_then(|item| item.from.as_deref()));
    let to = args
        .to
        .as_deref()
        .or_else(|| file.and_then(|item| item.to.as_deref()));
    match (kind, from, to) {
        (None, None, None) => Ok(None),
        (Some(kind), Some(from), Some(to)) => Ok(Some(ResolvedGradient {
            kind,
            from: parse_color(from)?,
            to: parse_color(to)?,
        })),
        _ => Err(QrError::Output(
            "gradient needs --gradient, --from, and --to".to_string(),
        )),
    }
}

fn resolve_frame(args: &Args, file: Option<&ThemeFile>) -> Result<Option<ResolvedFrame>, QrError> {
    let thickness = args.frame.or_else(|| file.and_then(|item| item.frame));
    let color = args
        .frame_color
        .as_deref()
        .or_else(|| file.and_then(|item| item.frame_color.as_deref()));
    let label = args
        .label
        .clone()
        .or_else(|| file.and_then(|item| item.label.clone()));
    match thickness {
        None if color.is_some() || label.is_some() => Err(QrError::Output(
            "border color and text need --frame or --border-width".to_string(),
        )),
        None => Ok(None),
        Some(thickness) => Ok(Some(ResolvedFrame {
            thickness,
            color: parse_color(color.unwrap_or("#000000"))?,
            label,
        })),
    }
}

fn read_logo(path: &str) -> Result<Vec<u8>, QrError> {
    if path.contains("://") {
        return Err(QrError::Output("logo must be a local file".to_string()));
    }
    fs::read(path).map_err(QrError::Io)
}
