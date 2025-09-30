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
  checksum64    = 'CHECKSUM64_PLACEHOLDER'
  checksumType64= 'sha256'
}

Install-ChocolateyZipPackage @packageArgs

# Create shim for the executable
$exePath = Join-Path $toolsDir "dororong-v$version-x86_64-pc-windows-msvc\dororong.exe"
Install-ChocolateyShim -Name 'dororong' -Path $exePath