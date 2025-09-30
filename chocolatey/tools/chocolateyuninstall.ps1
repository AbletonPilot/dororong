$ErrorActionPreference = 'Stop'

$packageName = 'dororong'

# Remove the shim
Uninstall-ChocolateyShim -Name 'dororong'

Write-Host "dororong has been uninstalled successfully." -ForegroundColor Green