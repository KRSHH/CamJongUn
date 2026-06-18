$ErrorActionPreference = "Stop"

Push-Location "$PSScriptRoot\.."
try {
    cargo build
    cargo test
    cargo run -p camjongunctl -- doctor
} finally {
    Pop-Location
}
