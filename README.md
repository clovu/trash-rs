# rust-template

A simple Rust project template to kickstart your development.

## Features

- **Environment Variable Management**: Uses the `dotenv` crate to load environment variables from a `.env` file.
- **Pre-commit Hooks**: Configured with `pre-commit` to ensure code quality and formatting.
- **CI/CD Integration**: Includes a GitHub Actions workflow for building, linting, and testing the project.
- **Code Formatting**: Enforces consistent code style with `rustfmt`.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- [cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)
- [pre-commit](https://pre-commit.com/) (optional, for pre-commit hooks)

## Getting Started

1. Clone the repository:

   ```sh
   git clone https://github.com/your-username/rust-template.git
   cd rust-template
   ```

2. Install dependencies:

   ```sh
   cargo build
   ```

3. Run the project:

   ```sh
   cargo run
   ```

4. Run tests:

   ```sh
   cargo test
   ```

## License

MIT License © 2025 [Clover You](https://github.com/clovu)
