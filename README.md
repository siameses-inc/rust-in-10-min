# rust-in-10-min

A small Rust playground for learning core language concepts and module layout.

## Build and run
- Build (debug): `cargo build`
- Build (release): `cargo build --release`
- Run: `cargo run`
- Run with args: `cargo run -- <args>`
- Check (fast typecheck): `cargo check`

## Tests
- Run all tests: `cargo test`
- Run a single test by name (substring): `cargo test <test_name>`
- Run a single test in a module: `cargo test <module_path>::<test_name>`
- Run tests in a file/module: `cargo test <module_path>`

## Usage example
```bash
cargo run
```

## Code layout
- `src/main.rs`: entry point and small example usage
- `src/domain/`: domain modules (e.g. calculator logic)

## Contributing
- Keep changes small and focused.
- Run `cargo fmt --all` before committing.
- Run `cargo clippy --all-targets --all-features -- -D warnings` before submitting.

## License
TBD
