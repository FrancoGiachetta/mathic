# Contributing to Mathic

Thanks for your interest in contributing! This guide covers how to set up the project, the workspace layout, and what is checked before opening a pull request.

## Development Setup

### Prerequisites

**LLVM/MLIR 21** is required. See [README.md](README.md) for install instructions. After installing, export the toolchain prefixes:

```sh
export LLVM_SYS_211_PREFIX=/path/to/llvm-21
export MLIR_SYS_210_PREFIX=/path/to/llvm-21
export TABLEGEN_210_PREFIX=/path/to/llvm-21
```

On macOS with Homebrew, `env.sh` exports these for you (run `source env.sh`).

The Rust toolchain is pinned in `rust-toolchain.toml` (1.94.0, with `rustfmt` and `clippy`); `rustup` picks it up automatically.

### Workspace layout

| Path        | Description                                        |
|-------------|----------------------------------------------------|
| `src/`      | The `mathic` compiler library                      |
| `euler/`    | The CLI binary (`euler new`, `euler run`)          |
| `Dialects/` | The custom `symbolic` MLIR dialect, written in C++ |
| `docs/`     | Architecture and compilation process documentation |
| `tests/`    | Integration tests                                  |

## Getting Started

1. Fork and clone the repository.
2. Set up the LLVM/MLIR prerequisites above.
3. Build with `cargo build`.
4. Run the test suite with `make test` (requires [cargo-nextest](https://nexte.st/)).

## Before Opening a PR

Run these locally; CI runs the same checks:

```sh
make fmt        # format Rust and the C++ dialect
make check      # cargo fmt --check + cargo clippy with -D warnings
make test       # full test suite (cargo nextest)
```

- Keep the code free of warnings — `make check` fails on any clippy warning (`-D warnings`).
- CI also checks for unused dependencies with [cargo-machete](https://github.com/bnjbvr/cargo-machete).
- When you change behavior, keep the docs in `docs/` (and this README) in sync.

## Where Things Live

- `src/parser/` — lexer, AST, and the recursive descent parser.
- `src/lowering/` — AST to MATHIR (Mathic's intermediate representation).
- `src/codegen/` — MATHIR to MLIR, using the `symbolic` dialect.
- `src/executor/` — the JIT execution engine.
- `tests/` — integration tests (`rstest`); `tests/fixtures/` holds expected-error cases.

## License

By contributing, you agree that your contributions are licensed under the [Apache-2.0](LICENSE) license.