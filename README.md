# trash-rs

`trash-rs` is an open-source Rust workspace that provides:

- `trash-rs`: a reusable library crate
- `trash-rs-cli`: a command-line tool built on top of the library

## Crates.io packages

- Library: [`trash-rs`](https://crates.io/crates/trash-rs)
- CLI: [`trash-rs-cli`](https://crates.io/crates/trash-rs-cli)

## Repository layout

- [crates/trash-rs/](crates/trash-rs/) — library source and crate README
- [crates/trash-rs-cli/](crates/trash-rs-cli/) — CLI source and crate README

> For user-facing install/usage docs, read each crate README above (these are what crates.io displays).

## Workspace development

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features --tests --benches -- -D warnings
cargo test --workspace --all-features
```

Run CLI from source:

```bash
cargo run -p trash-rs-cli -- --help
```

## License

MIT License © 2026 [Clover You](https://github.com/clovu)
