param(
  [string]$Version = "dev",
  [string]$Name = ""
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

if (-not $Name) {
  $runtimeInfo = [System.Runtime.InteropServices.RuntimeInformation]
  $osPlatform = [System.Runtime.InteropServices.OSPlatform]
  $osName = if ($runtimeInfo::IsOSPlatform($osPlatform::Windows)) {
    "windows-x64"
  } elseif ($runtimeInfo::IsOSPlatform($osPlatform::OSX)) {
    "macos-local"
  } else {
    "linux-x64"
  }
  $Name = "camjongun-$Version-$osName"
}

cargo test --workspace
cargo build --workspace --release

$Stage = Join-Path "dist" $Name
New-Item -ItemType Directory -Force -Path $Stage | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $Stage "bin") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $Stage "lib") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $Stage "include") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $Stage "bindings") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $Stage "docs") | Out-Null

Copy-Item README.md,IMPLEMENTATION_STATUS.md,PACKAGING_CONTRACT.md,identity-rules.md -Destination (Join-Path $Stage "docs")
Copy-Item include\* -Destination (Join-Path $Stage "include") -Recurse
Copy-Item bindings\* -Destination (Join-Path $Stage "bindings") -Recurse
Get-ChildItem (Join-Path $Stage "bindings") -Recurse -Directory |
  Where-Object { $_.Name -in @("bin", "obj", "node_modules", "__pycache__") } |
  Remove-Item -Recurse -Force
Copy-Item artifacts-needed -Destination (Join-Path $Stage "docs") -Recurse
Copy-Item native -Destination (Join-Path $Stage "docs") -Recurse

Get-ChildItem target\release -File -ErrorAction SilentlyContinue |
  Where-Object { $_.Name -in @("camjongunctl.exe", "camjongun-installer-helper.exe", "camjongunctl", "camjongun-installer-helper") } |
  Copy-Item -Destination (Join-Path $Stage "bin")

Get-ChildItem target\release -File -ErrorAction SilentlyContinue |
  Where-Object { $_.Name -match "^(camjongun_ffi|libcamjongun_ffi)\.(dll|lib|a|so|dylib)$" } |
  Copy-Item -Destination (Join-Path $Stage "lib")

"CamJongUn $Version" | Set-Content (Join-Path $Stage "PACKAGE.txt")
"This package contains the Rust SDK, C ABI artifacts, language bindings, CLI, installer-helper shell, headers, and docs." | Add-Content (Join-Path $Stage "PACKAGE.txt")
"Native OS camera driver artifacts are still contract-only until platform backend builds are wired." | Add-Content (Join-Path $Stage "PACKAGE.txt")

$Archive = Join-Path "dist" "$Name.zip"
if (Test-Path $Archive) {
  Remove-Item $Archive
}
Compress-Archive -Path (Join-Path $Stage "*") -DestinationPath $Archive
Write-Host "Packaged $Archive"
