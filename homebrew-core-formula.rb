class Dororong < Formula
  desc "Fun terminal animation app with dancing characters"
  homepage "https://github.com/AbletonPilot/dororong"
  url "https://github.com/AbletonPilot/dororong/archive/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"  # 실제 SHA256으로 교체 필요
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/dororong", "--help"
  end
end