<# 
  .SYNOPSIS
  Search for Tauri plugins in the crates.io registry.

  .NOTES
  Plugins can be included or excluded from the search manually
  by adding them to the $Include or $Exclude arrays.

  If you want to include or exclude a specific crate,
  open a pull request adding its name to the respective array.

  To edit metadata (e.g. description, repository, etc.) of a plugin,
  update your Cargo.toml file and publish a new version to crates.io.
#>

param(
  [switch]$Pretty
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$Command = 'cargo run -p cli --release -- search'

$Include = @()

$Exclude = @('tauri-plugin')

if ($Pretty) {
  $Command += ' --pretty'
}

foreach ($item in $Include) {
  $Command += " -i $item"
}

foreach ($item in $Exclude) {
  $Command += " -e $item"
}

Invoke-Expression $Command