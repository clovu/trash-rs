# trash-rs

`trash-rs` is a Rust library for moving files/directories to the system Trash safely (instead of permanently deleting them).

## Status

- macOS: implemented
- Windows: planned
- Linux: planned

## Installation

```bash
cargo add trash-rs
```

## Quick start

```rust
use trash_rs::trash;

fn main() -> trash_rs::Result<()> {
    trash("/path/to/file_or_dir")
}
```

Batch operation:

```rust
use trash_rs::trash_all;

fn main() -> trash_rs::Result<()> {
    trash_all(["/tmp/a.txt", "/tmp/b.txt"])
}
```

## Errors

Public API returns `trash_rs::Result<()>` and uses `trash_rs::TrashError`.

## License

MIT License © 2026 [Clover You](https://github.com/clovu)
