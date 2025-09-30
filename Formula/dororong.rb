class Dororong < Formula
  desc "Dororong will be dancing for you! 도로롱이 당신을 위해 춤을 춰줄 거예요!"
  homepage "https://github.com/AbletonPilot/dororong"
  version "0.1.2"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/AbletonPilot/dororong/releases/download/v#{version}/dororong-macos-aarch64"
      sha256 "SHA256_FOR_AARCH64" # This will be automatically updated by CI
    else
      url "https://github.com/AbletonPilot/dororong/releases/download/v#{version}/dororong-macos-x86_64"
      sha256 "SHA256_FOR_X86_64" # This will be automatically updated by CI
    end
  end

  def install
    if Hardware::CPU.arm?
      bin.install "dororong-macos-aarch64" => "dororong"
    else
      bin.install "dororong-macos-x86_64" => "dororong"
    end
  end

  test do
    system "#{bin}/dororong", "--help"
  end
end