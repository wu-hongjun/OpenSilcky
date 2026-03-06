# Plan 004 — macOS ARM64 .dmg Release Build

## Context

The current release workflow builds for 3 targets (macOS ARM64, macOS Intel, Linux) and packages everything as `.tar.gz`. Most Mac users don't know how to deal with tar.gz files. Since the primary audience is macOS Apple Silicon users, we should:

1. Build only `aarch64-apple-darwin` in CI (save CI credits)
2. Package as `.dmg` for easy drag-and-drop install
3. Drop the x86_64-apple-darwin and x86_64-unknown-linux-gnu matrix entries
4. Keep build-from-source instructions in README for other platforms

## Changes

### 1. Rewrite `.github/workflows/release.yml`

Remove the 3-target matrix. Single job on `macos-latest` (Apple Silicon):

- Single target (no matrix) — saves 2/3 of CI runner time
- `create-dmg` (Homebrew) to build the `.dmg`
- Merged build + release into one job (no artifact upload/download needed)
- `.dmg` contains `statuslight`, `statuslightd`, and FFI artifacts

### 2. Update `README.md` — Install section

Added a note that releases are macOS ARM64 only, with build-from-source instructions for other platforms.

### 3. Save plan to `docs/plans/004-dmg-release.md`

Per project convention.

## Files Modified

| File | Action |
|------|--------|
| `.github/workflows/release.yml` | Rewritten — single macOS ARM64 job, .dmg packaging |
| `README.md` | Updated — added Install section with .dmg download note |
| `docs/plans/004-dmg-release.md` | Created (this file) |

## Verification

1. Tag a new version (e.g., `v0.1.1`) to trigger the workflow
2. Workflow runs on a single `macos-latest` runner
3. GitHub Release contains a `.dmg` file
4. Download and mount the `.dmg` — contains `statuslight` and `statuslightd` binaries
5. `./statuslight devices` runs correctly from the mounted volume
