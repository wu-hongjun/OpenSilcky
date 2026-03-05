# Contributing

## Development Setup

1. Install Rust stable via [rustup](https://rustup.rs/)
2. Install system dependencies (see [Installation](../getting-started/installation.md))
3. Clone and build:

```bash
git clone https://github.com/openslicky/openslicky.git
cd openslicky
cargo build --workspace
cargo test --workspace
```

## Code Standards

### Formatting & Linting

```bash
cargo fmt --all           # format code
cargo clippy --workspace -- -D warnings  # lint (warnings are errors)
```

Run both before every commit.

### Error Handling

| Crate | Strategy |
|-------|----------|
| `slicky-core` | `thiserror` enum, `Result<T, SlickyError>` |
| `slicky-cli` | `anyhow::Result` with `.context()` |
| `slicky-daemon` | `anyhow` internally, map to HTTP status + JSON errors |
| `slicky-ffi` | Integer error codes, `catch_unwind` around everything |

**Never** use `.unwrap()` or `.expect()` in library code (`slicky-core`).

### Naming

- Crates: `slicky-*` (kebab-case)
- Types: `PascalCase`
- Functions: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- FFI exports: `slicky_` prefix

### Documentation

- All public types and functions need `///` doc comments
- Module-level `//!` doc comments in each file
- Include `# Examples` for user-facing APIs

### Testing

- Unit tests in `#[cfg(test)] mod tests {}` blocks
- Tests must not require a physical device
- Use descriptive `assert_eq!` messages

```rust
assert_eq!(result, expected, "from_hex should parse uppercase hex");
```

## Git Workflow

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add support for 3-char hex colors
fix: handle device disconnect during write
refactor: extract report building into protocol module
docs: add FFI usage example for Swift
test: add edge case tests for Color::from_hex
chore: update hidapi to 2.6
```

### Branches

- `main` — stable, always builds
- `feat/description` — feature branches
- `fix/description` — bug fix branches

### Pull Requests

- One logical change per PR
- All CI checks must pass (fmt, clippy, test, build)
- Include a test plan in the PR description
