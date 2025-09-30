class Dororong < Formula
  desc "Fun terminal animation app with dancing characters"
  homepage "https://github.com/AbletonPilot/dororong"
  url "https://github.com/AbletonPilot/dororong/archive/v0.1.0.tar.gz"
  sha256 "6e722d248763227b968c944c2bd69faaf9773ccd1e8752857620c932c30a613b" # Replace with actual SHA256 checksum
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/dororong", "--help"
  end
end