# Vendored OBS-Derived Source

This directory contains OBS-derived source staged for CamJongUn virtual camera
backends.

## Layout

- `platform/`: Windows, macOS, and Linux virtual camera backend source copied
  from OBS plugin areas.
- `shared/`: OBS shared support code needed by those backends.
- `deps/`: OBS dependency code needed by those backends.

## Rules

- Keep this tree as close to upstream OBS as possible.
- Do not put CamJongUn SDK behavior here.
- Do not hand-edit OBS identities here unless the change is unavoidable.
- Prefer Rust adapters, generated templates, or documented patches.
- Record unavoidable local changes in `../../patches/`.

The Rust SDK layer lives in `../../crates/`.
