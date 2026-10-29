class Qrbin < Formula
  desc "Offline-first QR generator with no telemetry and no network access"
  homepage "https://pesty.sh"
  license "MIT"
  head "https://github.com/pestyesky/qrbin.git", branch: "master"

  # After a tag is on GitHub, add these two lines and install without --HEAD.
  # The sha256 is of GitHub's tarball, not of a local git archive.
  # url "https://github.com/pestyesky/qrbin/archive/refs/tags/0.1.8.tar.gz"
  # sha256 "PASTE_SHA256_HERE"

  depends_on "rust" => :build

  def fetch
    system "cargo", "fetch", *std_cargo_fetch_args
  end

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "qrbin", shell_output("#{bin}/qrbin --version")
    png = IO.popen([bin/"qrbin", "--size", "64", "--format", "png", "hello"], "rb", &:read)
    assert png.start_with?("\x89PNG".b), "qrbin did not write a PNG"
  end
end
