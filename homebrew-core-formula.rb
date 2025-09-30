class Dororong < Formula
  desc "Fun terminal animation app with dancing characters"
  homepage "https://github.com/AbletonPilot/dororong"
  url "https://github.com/AbletonPilot/dororong/archive/v0.1.0.tar.gz"
  sha256 "83f5f11a8b46e6bb0acfbd817db1ad9f61a903dcae43ab9998011c9844dde7f5" # Replace with actual SHA256 checksum
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/dororong", "--help"
  end
end