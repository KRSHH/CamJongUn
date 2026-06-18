$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

.\scripts\fetch-native-deps.ps1

cargo build -p camjongun-installer-helper --release

$Build64 = Join-Path $Root "build\native-windows-x64"
$Stage = Join-Path $Root "artifacts\native\windows"
New-Item -ItemType Directory -Force -Path $Build64 | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $Stage "bin") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $Stage "docs") | Out-Null

cmake -S native\windows\directshow -B $Build64 -A x64
cmake --build $Build64 --config Release --target camjongun-virtualcam-module

Copy-Item "$Build64\Release\camjongun-virtualcam-module64.dll" -Destination (Join-Path $Stage "bin") -Force
Copy-Item "target\release\camjongun-installer-helper.exe" -Destination (Join-Path $Stage "bin") -Force
Copy-Item "native\windows\README.md" -Destination (Join-Path $Stage "docs") -Force
Copy-Item "artifacts-needed\windows\README.md" -Destination (Join-Path $Stage "docs\artifacts-needed-windows.md") -Force

if ($env:CAMJONGUN_BUILD_X86 -eq "1") {
  $Build32 = Join-Path $Root "build\native-windows-x86"
  New-Item -ItemType Directory -Force -Path $Build32 | Out-Null
  cmake -S native\windows\directshow -B $Build32 -A Win32
  cmake --build $Build32 --config Release --target camjongun-virtualcam-module
  Copy-Item "$Build32\Release\camjongun-virtualcam-module32.dll" -Destination (Join-Path $Stage "bin") -Force
}

@"
# CamJongUn Windows Native Package

Contains CamJongUn DirectShow virtual camera module binaries and installer helper.
Register the DLLs through CamJongUn installer/helper flow, not manually.
"@ | Set-Content (Join-Path $Stage "README.md")
