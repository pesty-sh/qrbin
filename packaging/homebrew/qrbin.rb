class Qrbin < Formula
  desc "Offline-first QR generator with no telemetry and no network access"
  homepage "https://pesty.sh"
  license "MIT"
  # Source tarball for tag v0.1.9. The sha256 was computed from two
  # separate downloads of this URL. There is no bottle.
  url "https://github.com/pesty-sh/qrbin/archive/refs/tags/v0.1.9.tar.gz"
  sha256 "afc109f18bc91139a26d1f881b1e590112ad53ad38e17b9d4dd2ab78c1a0e81e"
  head "https://github.com/pesty-sh/qrbin.git", branch: "master"

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
