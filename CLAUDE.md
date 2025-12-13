# sysdoc

## Project Overview

`sysdoc` is a Rust-based system documentation tool currently in early development.

## Project Structure

This is a Cargo workspace with the following structure:
- `/sysdoc/` - Main crate containing the sysdoc application
- `Cargo.toml` - Workspace configuration at the root

## Technology Stack

- **Language**: Rust (Edition 2021)
- **Build System**: Cargo workspace

## Development Guidelines

### Code Style

- Follow standard Rust conventions and idioms
- Use `rustfmt` for code formatting
- Use `clippy` for linting
- Prefer descriptive variable and function names
- Add documentation comments (`///`) for public APIs

### Testing

- Write unit tests in the same file as the code being tested
- Use integration tests in the `tests/` directory for end-to-end functionality
- Run tests with `cargo test`
- Aim for meaningful test coverage of core functionality

### Building

```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run the application
cargo run

# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt
```

## Architecture

[To be defined as the project develops]

## Dependencies

Currently, the project has minimal dependencies. New dependencies should be:
- Well-maintained and widely used in the Rust ecosystem
- Added to `[workspace.dependencies]` in the root `Cargo.toml` for shared dependencies
- Justified with a clear use case

## Current Status

Early development stage - the project is being scaffolded and initial architecture decisions are being made.
