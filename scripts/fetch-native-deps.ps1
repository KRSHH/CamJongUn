$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
$LibDshow = Join-Path $Root "vendor\obs\deps\libdshowcapture\src"

if (
  (Test-Path (Join-Path $LibDshow "source\output-filter.hpp")) -and
  (Test-Path (Join-Path $LibDshow "external\capture-device-support\Library\EGAVResult.cpp"))
) {
  Write-Host "libdshowcapture already present"
  exit 0
}

if (Test-Path $LibDshow) {
  Remove-Item -LiteralPath $LibDshow -Recurse -Force
}

git clone --depth 1 --recurse-submodules --shallow-submodules https://github.com/obsproject/libdshowcapture.git $LibDshow
