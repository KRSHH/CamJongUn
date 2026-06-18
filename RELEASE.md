# Release Flow

CamJongUn releases are driven by GitHub Actions.

## What A Release Ships Today

- Rust SDK crate source.
- C ABI dynamic/static library from `camjongun-ffi`.
- C header from `include/camjongun/camjongun.h`.
- `camjongunctl` CLI.
- `camjongun-installer-helper` helper shell.
- Packaging and native artifact contract docs.

The release package does not yet contain working Windows DirectShow modules,
macOS Camera Extension/DAL bundles, or Linux `v4l2loopback` integration. Those
remain the next native backend phase.

## Manual Local Package

```powershell
.\scripts\package-release.ps1 -Version v0.1.0
```

## GitHub Release

After the repository exists on GitHub and `gh auth login` has been completed:

```powershell
.\scripts\publish-github.ps1 -Repo https://github.com/<owner>/<repo>.git -Tag v0.1.0
```

The script initializes the repo if needed, commits the current tree, pushes the
main branch, pushes the tag, and lets `.github/workflows/release.yml` publish
the release artifacts.
