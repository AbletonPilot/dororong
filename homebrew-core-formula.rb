class Dororong < Formula
  desc "Fun terminal animation app with dancing characters"
  homepage "https://github.com/AbletonPilot/dororong"
  url "https://github.com/AbletonPilot/dororong/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "6e722d248763227b968c944c2bd69faaf9773ccd1e8752857620c932c30a613b"
  license "MIT"
  head "https://github.com/AbletonPilot/dororong.git", branch: "main"

  livecheck do
    url :stable
    strategy :github_latest
  end

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "dororong", shell_output("#{bin}/dororong --help")
    assert_match "v0.1.0", shell_output("#{bin}/dororong --version")
  end
end
