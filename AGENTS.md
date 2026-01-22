# Agent Guidance for rust-in-10-min

This file is for agentic coding tools (including ChatGPT/Cursor/Copilot).
Use it as the single source of truth for commands and style in this repo.

## Project snapshot
- Language: Rust (edition 2024)
- Build tool: Cargo
- Source root: `src/`
- Entry point: `src/main.rs`
- Manifest: `Cargo.toml`
- Build artifacts: `target/`
- Crate type: binary (no `lib.rs`)
- Dependencies: none in `Cargo.toml`; prefer std before adding crates
- Modules: `src/domain/mod.rs` and `src/domain/basic_calculator.rs`

## Commands (build/lint/test)
### Build & run
- Build debug: `cargo build`
- Build release: `cargo build --release`
- Run: `cargo run`
- Run with args: `cargo run -- <args>`
- Check (fast typecheck): `cargo check`

### Lint & format
- Format check: `cargo fmt --all -- --check`
- Format write: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`

### Tests
- Run all tests: `cargo test`
- Run tests with output: `cargo test -- --nocapture`
- Run a single test by name (substring): `cargo test <test_name>`
- Run a single test in a module: `cargo test <module_path>::<test_name>`
- Run tests in a specific file/module: `cargo test <module_path>`
- Run only doc tests: `cargo test --doc`
- Run tests in release mode: `cargo test --release`

### Clean
- Clean build artifacts: `cargo clean`

## Code style guidelines
### Formatting and linting
- Always keep code `rustfmt`-clean; run `cargo fmt` after edits.
- Keep code `clippy`-clean; treat warnings as errors (`-D warnings`).
- Prefer explicit formatting over manual alignment or spacing tricks.
- Avoid trailing whitespace and unused variables.
- Use underscore prefix for intentionally unused locals (e.g., `_value`).

### Imports
- Place all imports at the top of the file.
- Group imports by crate, then alphabetically within group.
- Avoid inline imports inside functions; use `use` at module scope.
- Use `crate::` paths for internal modules when possible.
- Avoid glob imports except in tests.
- In test modules, prefer `use super::...` for local items.

### Naming
- Functions and variables: `snake_case`.
- Types and traits: `PascalCase`.
- Constants: `SCREAMING_SNAKE_CASE`.
- Modules and files: `snake_case`.
- Use descriptive names over abbreviations.

### Types and ownership
- Prefer explicit types where clarity is needed; allow inference when obvious.
- Avoid unnecessary `clone()`; use references and borrowing where feasible.
- Use `&str` for inputs when ownership is not required.
- Use `String` for owned, mutable text.
- Favor iterators over indexed loops.
- Prefer slices (`&[T]`) over owned `Vec<T>` for inputs.
- Avoid `mut` unless required for clarity.

### Error handling
- Use `Result<T, E>` for fallible operations; return early with `?`.
- Prefer concrete error types; use `Box<dyn std::error::Error>` only for prototypes.
- Avoid panics in normal control flow; use `expect`/`unwrap` only for tests or
  when invariants are guaranteed and documented.
- Use `Option<T>` for absence and convert to `Result` only at boundaries.
- Prefer returning errors from helpers instead of handling in `main`.

### Functions and APIs
- Keep functions small and single-purpose.
- Use clear parameter names; avoid abbreviations unless standard.
- Prefer pure functions; minimize side effects.
- Document non-obvious behavior with concise comments.
- Keep `main` thin; move logic to modules as it grows.
- Traits should use verbs (`Calculator`) and implementations should be nouns.

### Tests
- Place unit tests in the same module with `#[cfg(test)]`.
- Name tests by behavior: `does_x_when_y` or `returns_x_for_y`.
- Use `assert_eq!`/`assert!` with descriptive messages if needed.
- Prefer deterministic tests; avoid time or randomness unless seeded.
- Keep test helpers private to the test module.

### Documentation
- Use `///` doc comments for public functions or modules.
- Keep examples short and compile-ready.
- Prefer rustdoc examples over inline comments for behavior.

### Collections and iteration
- Prefer iterators and `map`/`filter` over manual loops when clear.
- Use `iter()` for borrowing, `iter_mut()` for mutation, and `into_iter()` for
  taking ownership.
- Avoid indexing unless bounds are guaranteed.

## Repo conventions
- Keep `src/main.rs` minimal; move logic into modules when it grows.
- Introduce new modules under `src/` and declare with `mod`.
- Avoid adding dependencies unless necessary; keep `Cargo.toml` lean.
- Keep module files in `src/` using `mod_name.rs` or `mod_name/mod.rs`.
- Prefer `pub(crate)` over `pub` unless the API is intentionally public.
- Update `src/domain/mod.rs` when adding domain submodules.

## Local patterns
- Keep arithmetic/domain logic in `src/domain/` modules.
- Prefer simple, total functions for calculator-style operations.
- Keep public APIs small and focused; expose traits where useful.
- Use `println!` only in `main` or examples, not domain modules.

## Tooling/automation expectations
- Do not commit or push unless explicitly asked by the user.
- Explain intent before making changes.
- When reviewing changes on GitHub PRs, use `gh` and include "Added by AI"
  in comments, and reference the file path where suggesting changes.
- Do not add new CI/config files unless requested.

## Cursor/Copilot rules
- No `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md`
  found in this repo at time of writing.

## Example test commands
- Run a single test in a module:
  `cargo test my_module::tests::returns_expected`
- Run a single test by substring:
  `cargo test returns_expected`
- Run tests in a file (module path):
  `cargo test my_module`
- Run a single test in `src/main.rs`:
  `cargo test tests::adding`

## Notes for agents
- Keep changes minimal and focused; prefer small diffs.
- Preserve existing patterns unless a change is explicitly requested.
- Prefer standard library solutions before adding crates.
- Write ASCII-only content unless file already uses Unicode.
- If a file already uses Unicode, keep additions consistent.
