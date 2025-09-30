$ErrorActionPreference = 'Stop'

$packageName = 'dororong'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$version = '0.1.0'
$url64 = "https://github.com/AbletonPilot/dororong/releases/download/v$version/dororong-v$version-x86_64-pc-windows-msvc.tar.gz"

$packageArgs = @{
  packageName   = $packageName
  fileFullPath  = Join-Path $toolsDir "dororong.tar.gz"
  url64bit      = $url64
  checksum64    = 'b41c98aa23a2da8f2a67ea209d6bf9aebc450acde4b140d4a2050ba823c171ca'
  checksumType64= 'sha256'
}

Get-ChocolateyWebFile @packageArgs

# Extract tar.gz
$extractDir = Join-Path $toolsDir "dororong-v$version-x86_64-pc-windows-msvc"
tar -xzf $packageArgs.fileFullPath -C $toolsDir
if (!(Test-Path $extractDir)) {
  Write-Error "Extraction failed: $extractDir not found"
}

# Create shim for the executable
$exePath = Join-Path $extractDir "dororong.exe"
if (Test-Path $exePath) {
  Install-ChocolateyShim -Name 'dororong' -Path $exePath
} else {
  Write-Error "dororong.exe not found at expected path: $exePath"
}