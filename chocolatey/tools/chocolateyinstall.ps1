$ErrorActionPreference = 'Stop'

$packageName = 'dororong'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$version = '0.1.0'
$url64 = "https://github.com/AbletonPilot/dororong/releases/download/v$version/dororong-v$version-x86_64-pc-windows-msvc.zip"

$packageArgs = @{
  packageName   = $packageName
  unzipLocation = $toolsDir
  url64bit      = $url64
  softwareName  = 'dororong*'
  checksum64    = '0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5'
  checksumType64= 'sha256'
}

Install-ChocolateyZipPackage @packageArgs

# Create shim for the executable
$exePath = Join-Path $toolsDir "dororong-v$version-x86_64-pc-windows-msvc\dororong.exe"
if (Test-Path $exePath) {
  Install-ChocolateyShim -Name 'dororong' -Path $exePath
} else {
  Write-Error "dororong.exe not found at expected path: $exePath"
}