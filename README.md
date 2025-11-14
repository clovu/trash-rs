# trash-rs

`trash-rs` is a Rust CLI tool to safely move files to the Trash. Currently, it supports **macOS**, with **Windows** and **Linux** support planned for future releases.

## Features

* Safely move files to the system Trash without permanent deletion.
* Lightweight and simple CLI interface.
* Preserves file paths and metadata.
* Designed for cross-platform support in the future.

## Usage

```bash
trash /path/to/your/file.txt
```

This will move the specified file to the Trash.

## Planned Features

* **Windows support**: Move files to the Recycle Bin.
* **Linux support**: Move files to the Trash according to the FreeDesktop.org standard.

## License

MIT License © 2025 [Clover You](https://github.com/clovu)
