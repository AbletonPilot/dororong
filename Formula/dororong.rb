class Dororong < Formula
  desc "Fun terminal animation app with dancing characters"
  homepage "https://github.com/AbletonPilot/dororong"
  url "https://github.com/AbletonPilot/dororong/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "b2272a71d124dae5373591535b94a449c0f643e322b339eaba113ea48b6f88c8"
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
    assert_match "0.1.1", shell_output("#{bin}/dororong --version")
  end
end

