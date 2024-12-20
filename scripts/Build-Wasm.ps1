$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

Get-ChildItem 'crates/wasm/pkg' -Recurse | Remove-Item

wasm-pack build 'crates/wasm' --release
