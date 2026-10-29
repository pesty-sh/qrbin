//! CLI checks for classic and themed output.
#![allow(clippy::expect_used)]

use std::io::Cursor;

use assert_cmd::Command;
use image::codecs::png::PngDecoder;
use image::{ColorType, ImageDecoder};
use predicates::prelude::*;

fn qrbin() -> Command {
    Command::cargo_bin("qrbin").expect("qrbin test binary")
}

#[test]
fn classic_png_stays_grayscale() {
    let output = qrbin()
        .args(["https://example.com", "--format", "png", "--size", "64"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let decoder = PngDecoder::new(Cursor::new(output)).expect("png");
    assert_eq!(decoder.color_type(), ColorType::L8);
    assert!(decoder.dimensions().0 >= 64);
}

#[test]
fn rounded_png_is_rgba_at_the_requested_size() {
    let output = qrbin()
        .args([
            "https://example.com",
            "--theme",
            "rounded",
            "--format",
            "png",
            "--size",
            "64",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let decoder = PngDecoder::new(Cursor::new(output)).expect("png");
    assert_eq!(decoder.color_type(), ColorType::Rgba8);
    assert_eq!(decoder.dimensions(), (64, 64));
}

#[test]
fn theme_file_renders_svg() {
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join("theme.toml");
    std::fs::write(&path, "theme = \"dots\"\nfg = \"112233\"\n").expect("theme file");

    qrbin()
        .args(["https://example.com", "--theme-file"])
        .arg(&path)
        .args(["--format", "svg", "--size", "64"])
        .assert()
        .success()
        .stdout(predicate::str::contains("<svg"));
}

#[test]
fn remote_logo_is_rejected_without_echoing_the_payload() {
    qrbin()
        .args([
            "https://example.com",
            "--theme",
            "classy",
            "--logo",
            "https://example.com/mark.png",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("logo must be a local file"))
        .stderr(predicate::str::contains("https://example.com").not());
}

#[test]
fn invalid_color_does_not_echo_the_payload() {
    qrbin()
        .args(["secret-payload", "--theme", "plain", "--fg", "zz"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid color"))
        .stderr(predicate::str::contains("secret-payload").not());
}

#[test]
fn style_flags_need_a_theme() {
    qrbin()
        .args(["https://example.com", "--fg", "111111"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("pass --theme"));
}

#[test]
fn gradient_and_frame_render() {
    qrbin()
        .args([
            "https://example.com",
            "--theme",
            "dots",
            "--gradient",
            "linear",
            "--from",
            "5b21b6",
            "--to",
            "2563eb",
            "--format",
            "svg",
            "--size",
            "64",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("<svg"));

    qrbin()
        .args([
            "https://example.com",
            "--theme",
            "plain",
            "--frame",
            "12",
            "--frame-color",
            "111111",
            "--label",
            "qrbin",
            "--format",
            "png",
            "--size",
            "64",
        ])
        .assert()
        .success()
        .stdout(predicate::function(|bytes: &[u8]| {
            bytes.starts_with(b"\x89PNG")
        }));

    qrbin()
        .args([
            "https://example.com",
            "--theme",
            "rounded",
            "--shape",
            "circle",
            "--ecc",
            "high",
            "--format",
            "png",
            "--size",
            "64",
        ])
        .assert()
        .success()
        .stdout(predicate::function(|bytes: &[u8]| {
            bytes.starts_with(b"\x89PNG")
        }));
}

#[test]
fn local_logo_renders() {
    let dir = tempfile::tempdir().expect("temp dir");
    let logo = dir.path().join("mark.png");
    image::RgbImage::from_pixel(8, 8, image::Rgb([9, 9, 9]))
        .save(&logo)
        .expect("logo");

    qrbin()
        .args(["https://example.com", "--theme", "classy", "--logo"])
        .arg(&logo)
        .args(["--ecc", "high", "--format", "png", "--size", "64"])
        .assert()
        .success()
        .stdout(predicate::function(|bytes: &[u8]| {
            bytes.starts_with(b"\x89PNG")
        }));
}

#[test]
fn gradient_needs_both_stops() {
    qrbin()
        .args([
            "https://example.com",
            "--theme",
            "dots",
            "--gradient",
            "radial",
            "--from",
            "112233",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("gradient needs"));
}

#[test]
fn border_width_and_text_are_flags() {
    qrbin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--border-width"))
        .stdout(predicate::str::contains("--text"));

    qrbin()
        .args([
            "https://example.com",
            "--theme",
            "plain",
            "--text",
            "Scan Me!",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--border-width"));

    qrbin()
        .args([
            "https://example.com",
            "--theme",
            "dots",
            "--shape",
            "circle",
            "--border-width",
            "24",
            "--border-color",
            "6d89a8",
            "--text",
            "Scan Me!",
            "--format",
            "png",
            "--size",
            "128",
        ])
        .assert()
        .success()
        .stdout(predicate::function(|bytes: &[u8]| {
            bytes.starts_with(b"\x89PNG")
        }));
}

struct LibraryExample {
    file: &'static str,
    args: &'static [&'static str],
}

/// Commands that produced a viewable PNG. Broken runs are omitted: `library/mark.png`
/// does not exist, `library/round.png` was a typo for `rounded.png`, a glued `\--ecc`,
/// a doubled `qrbin` invocation, and an unclosed `"Scan Me` quote.
const LIBRARY_EXAMPLES: &[LibraryExample] = &[
    LibraryExample {
        file: "classic.png",
        args: &["--format", "png", "--size", "512"],
    },
    LibraryExample {
        file: "plain.png",
        args: &["--theme", "plain", "--format", "png", "--size", "512"],
    },
    LibraryExample {
        file: "rounded.png",
        args: &["--theme", "rounded", "--format", "png", "--size", "512"],
    },
    LibraryExample {
        file: "dots.png",
        args: &["--theme", "dots", "--format", "png", "--size", "512"],
    },
    LibraryExample {
        file: "classy.png",
        args: &["--theme", "classy", "--format", "png", "--size", "512"],
    },
    LibraryExample {
        file: "classy-colors.png",
        args: &[
            "--theme", "classy", "--fg", "1a1a1a", "--bg", "f7f3ea", "--format", "png", "--size",
            "512",
        ],
    },
    LibraryExample {
        file: "dots-gradient-linear.png",
        args: &[
            "--theme",
            "dots",
            "--gradient",
            "linear",
            "--from",
            "5b21b6",
            "--to",
            "2563eb",
            "--ecc",
            "high",
            "--format",
            "png",
            "--size",
            "512",
        ],
    },
    LibraryExample {
        file: "dots-gradient-radial.png",
        args: &[
            "--theme",
            "dots",
            "--gradient",
            "radial",
            "--from",
            "5b21b6",
            "--to",
            "2563eb",
            "--ecc",
            "high",
            "--format",
            "png",
            "--size",
            "512",
        ],
    },
    LibraryExample {
        file: "rounded-circle.png",
        args: &[
            "--theme", "rounded", "--shape", "circle", "--ecc", "high", "--format", "png",
            "--size", "512",
        ],
    },
    LibraryExample {
        file: "classy-logo.png",
        args: &[
            "--theme",
            "classy",
            "--logo",
            "assets/logo.png",
            "--ecc",
            "high",
            "--format",
            "png",
            "--size",
            "512",
        ],
    },
    LibraryExample {
        file: "plain-frame.png",
        args: &[
            "--theme",
            "plain",
            "--frame",
            "12",
            "--frame-color",
            "111111",
            "--label",
            "qrbin",
            "--format",
            "png",
            "--size",
            "512",
        ],
    },
    LibraryExample {
        file: "logo-rounded.png",
        args: &[
            "--theme",
            "rounded",
            "--fg",
            "1a1a1a",
            "--bg",
            "f7f3ea",
            "--logo",
            "assets/logo.png",
            "--ecc",
            "high",
            "--format",
            "png",
            "--size",
            "1024",
        ],
    },
    LibraryExample {
        file: "logo-title.png",
        args: &[
            "--theme",
            "classy",
            "--fg",
            "1a1a1a",
            "--bg",
            "f7f3ea",
            "--logo",
            "assets/logo.png",
            "--ecc",
            "high",
            "--format",
            "png",
            "--size",
            "1024",
        ],
    },
    LibraryExample {
        file: "logo-linear-blue.png",
        args: &[
            "--theme",
            "dots",
            "--gradient",
            "linear",
            "--from",
            "1e3a8a",
            "--to",
            "38bdf8",
            "--bg",
            "f7f3ea",
            "--logo",
            "assets/logo.png",
            "--ecc",
            "high",
            "--format",
            "png",
            "--size",
            "1024",
        ],
    },
    LibraryExample {
        file: "logo-radial-blue.png",
        args: &[
            "--theme",
            "dots",
            "--gradient",
            "radial",
            "--from",
            "1e2a7a",
            "--to",
            "38bdf8",
            "--bg",
            "e7e3da",
            "--logo",
            "assets/logo.png",
            "--ecc",
            "high",
            "--format",
            "png",
            "--size",
            "1024",
        ],
    },
    LibraryExample {
        file: "logo-radial-ring.png",
        args: &[
            "--theme",
            "dots",
            "--gradient",
            "radial",
            "--from",
            "0a1020",
            "--to",
            "6d89a8",
            "--bg",
            "e7e3da",
            "--logo",
            "assets/logo.png",
            "--shape",
            "circle",
            "--border-width",
            "128",
            "--border-color",
            "6d89a8",
            "--text",
            "Scan Me!",
            "--ecc",
            "high",
            "--format",
            "png",
            "--size",
            "1024",
        ],
    },
    LibraryExample {
        file: "logo-rounded-circle.png",
        args: &[
            "--theme",
            "rounded",
            "--gradient",
            "radial",
            "--from",
            "171717",
            "--to",
            "737373",
            "--bg",
            "f5f5f5",
            "--logo",
            "assets/logo.png",
            "--shape",
            "circle",
            "--border-width",
            "128",
            "--border-color",
            "737373",
            "--text",
            "Scan Me!",
            "--ecc",
            "high",
            "--format",
            "png",
            "--size",
            "1024",
        ],
    },
];

#[test]
fn library_examples() {
    std::fs::create_dir_all("library").expect("library dir");
    for example in LIBRARY_EXAMPLES {
        let output = qrbin()
            .arg("https://example.com")
            .args(example.args)
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();
        assert!(
            output.starts_with(b"\x89PNG") && output.len() > 32,
            "{}",
            example.file
        );
        std::fs::write(format!("library/{}", example.file), &output).expect(example.file);
    }
}
