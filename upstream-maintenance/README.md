# Upstream Maintenance Policy

CamJongUn should treat OBS code as vendored upstream code.

## Rules

- Keep OBS-derived code in `vendor/obs/` as close to upstream OBS as possible.
- Put CamJongUn-specific API and product behavior in `library/`.
- Do not rename or casually reformat upstream OBS files.
- If a local patch to upstream code is unavoidable, store it in `patches/` with
  a short explanation.
- Prefer adapter files over invasive edits.

## Upgrade Flow

1. Import the new OBS virtual camera related source into a temporary comparison
   area.
2. Diff against the current `vendor/obs/` copy.
3. Replace upstream files wholesale where possible.
4. Reapply only documented patches from `patches/`.
5. Rebuild CamJongUn adapters.
6. Run `cargo test` from the CamJongUn root. The upstream contract tests should
   catch layout drift, renamed upstream markers, accidental OBS artifact reuse,
   registry compatibility breaks, and platform report regressions.
7. Let GitHub Actions run the Windows, macOS, and Linux contract jobs.
8. Verify Windows registration, macOS extension activation, and Linux
   `v4l2loopback` behavior.

## Why This Matters

OBS will keep changing its internals. CamJongUn should expose a stable developer
API while letting the copied OBS implementation be refreshed without a painful
fork merge every time.
