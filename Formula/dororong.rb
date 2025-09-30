class Dororong < Formula
  desc "Dororong will be dancing for you!"
  homepage "https://github.com/AbletonPilot/dororong"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/AbletonPilot/dororong/releases/download/v#{version}/dororong-macos-aarch64"
      sha256 "SHA256_FOR_AARCH64" # Replace with actual SHA256
    else
      url "https://github.com/AbletonPilot/dororong/releases/download/v#{version}/dororong-macos-x86_64"
      sha256 "SHA256_FOR_X86_64" # Replace with actual SHA256
    end
  end

  def install
    bin.install "dororong" => "dororong"
  end

  test do
    system "#{bin}/dororong", "--help"
  end
end