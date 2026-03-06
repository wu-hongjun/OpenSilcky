# CI/CD Setup — GitHub Actions

## Context

StatusLight has a working Rust workspace with 4 crates (core, cli, daemon, ffi) but no CI/CD. We need to:
1. Fix `.gitignore` to track `Cargo.lock` (required for reproducible binary builds)
2. Add a CI workflow that runs on every PR/push
3. Add a release workflow that builds binaries for macOS + Linux when a tag is pushed

## Changes

### 1. Fix `.gitignore` — remove `Cargo.lock`

**File:** `.gitignore`

Remove the `Cargo.lock` line. Per Rust convention, projects with binaries should commit their lock file.

Then `git add Cargo.lock`.

### 2. CI Workflow — `.github/workflows/ci.yml`

**Triggers:** push to `main`, all pull requests

**Job: `check`** on `ubuntu-latest`:
1. Checkout
2. Install Rust stable + clippy + rustfmt (via `dtolnay/rust-toolchain`)
3. Install system deps (`sudo apt-get install -y libhidapi-dev`)
4. Cache `~/.cargo` and `target/` (via `Swatinem/rust-cache`)
5. `cargo fmt --all --check`
6. `cargo clippy --workspace -- -D warnings`
7. `cargo test --workspace`
8. `cargo build --workspace`

### 3. Release Workflow — `.github/workflows/release.yml`

**Triggers:** push tag matching `v*` (e.g. `v0.1.0`)

**Build matrix:**

| Target | Runner | System Deps |
|--------|--------|-------------|
| `x86_64-apple-darwin` | `macos-13` | `brew install hidapi` |
| `aarch64-apple-darwin` | `macos-latest` (ARM) | `brew install hidapi` |
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` | `apt install libhidapi-dev` |

Per-target steps:
1. Checkout
2. Install Rust stable + target (`dtolnay/rust-toolchain`)
3. Install platform system deps
4. `cargo build --workspace --release --target $TARGET`
5. Package binaries (`statuslight`, `statuslightd`) + FFI lib + header into `statuslight-$TAG-$TARGET.tar.gz`
6. Upload artifact

**Final job** (`release`, `needs: build`):
1. Download all artifacts
2. Create GitHub Release via `softprops/action-gh-release` with all tarballs attached

### Files to create/modify

| File | Action |
|------|--------|
| `.gitignore` | Edit: remove `Cargo.lock` line |
| `.github/workflows/ci.yml` | Create |
| `.github/workflows/release.yml` | Create |

### Verification

- `cargo fmt --all --check` — clean
- `cargo clippy --workspace -- -D warnings` — clean
- `cargo test --workspace` — all pass
- Push commit, verify CI workflow runs on GitHub
