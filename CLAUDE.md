# OpenSlicky — Project Instructions

## Plans

- Every plan created during plan mode must be saved as a numbered file in `/docs/plans/` (e.g., `002-ci-cd-setup.md`, `003-config-file.md`).
- Plans are the source of truth for implementation decisions and should be committed alongside the code they describe.

## Code Standards

- See `/docs/plans/001-full-stack-scaffold.md` for full coding rules (formatting, linting, error handling, naming, testing, dependencies, git conventions).

## Quick Reference

- `cargo fmt --all` before every commit
- `cargo clippy --workspace -- -D warnings` — treat all warnings as errors
- `cargo test --workspace` — all tests must pass
- Conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`
