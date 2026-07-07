<div align="center">

# Mathic

*A programming language with built-in symbolic algebra, powered by LLVM/MLIR*

</div>

## Features

- **Symbolic expressions** — first-class symbolic algebra: declare symbolic variables, compose expressions with `+`, `-`, `*`, `/`, and evaluate with concrete values at runtime.
- **JIT compilation** — programs are compiled on-the-fly via MLIR/LLVM and executed natively.
- **MLIR-powered** — a custom MLIR dialect (`symbolic`) represents symbolic expressions, lowered to `arith` + `func` for native codegen.
- **Statically typed** — type-checked at compile time.

## Example

```rust
df main() i32 {
    sym x: expr<i32>;

    let a: expr<i32> = x + 5;
    let r1: i32 = eval(a, x, 10);

    let b: expr<i32> = 3 * x;
    let r2: i32 = eval(b, x, 10);

    let c: expr<i32> = 2 * x + 1;
    let r3: i32 = eval(c, x, 10);

    return r1 + r2 + r3;
}
```

Declares symbolic variables (`sym`), composes expressions with arithmetic, and evaluates them with concrete values at runtime.

## Quick Start

```bash
git clone https://github.com/FrancoGiachetta/mathic.git
cd mathic
cargo run examples/arithmetic/addition.mth
```

## Prerequisites

**LLVM/MLIR 21** is required. After installing, set:

```sh
export LLVM_SYS_211_PREFIX=/path/to/llvm-21
export MLIR_SYS_210_PREFIX=/path/to/llvm-21
export TABLEGEN_210_PREFIX=/path/to/llvm-21
```

<details>
<summary>Installing LLVM/MLIR (click to expand)</summary>

### macOS (Homebrew)

```bash
brew install llvm@21
export LLVM_SYS_211_PREFIX=$(brew --prefix llvm@21)
export MLIR_SYS_210_PREFIX=$(brew --prefix llvm@21)
export TABLEGEN_210_PREFIX=$(brew --prefix llvm@21)
```

You may also need:

```sh
export LIBRARY_PATH=/opt/homebrew/lib
```

### Building from source

> Requires ~6 GB RAM and ~20 GB disk space.

```bash
git clone https://github.com/llvm/llvm-project.git
cd llvm-project
git checkout llvmorg-21.1.7
cmake -G Ninja ../llvm \
    -DLLVM_ENABLE_PROJECTS="mlir" \
    -DCMAKE_BUILD_TYPE=RelWithDebInfo \
    -DLLVM_ENABLE_ASSERTIONS=On \
    -DLLVM_BUILD_LLVM_DYLIB=On \
    -DLLVM_LINK_LLVM_DYLIB=On \
    -DMLIR_BUILD_MLIR_C_DYLIB=On \
    -DLLVM_TARGETS_TO_BUILD=host \
    -DCMAKE_INSTALL_PREFIX=/opt/llvm-21 \
    -DLLVM_USE_LINKER=mold   # optional, faster with mold
ninja install
```

</details>

## Installation

```bash
cargo install mathic
```

Make sure `LLVM_SYS_211_PREFIX`, `MLIR_SYS_210_PREFIX` and `TABLEGEN_210_PREFIX` are set before building.

## Usage

```bash
euler <file>.mth
```

## Project

See [docs/](docs/README.md) for the full project structure and pipeline.
See [docs/dialects/Symbolic.md](docs/dialects/Symbolic.md) for the symbolic dialect reference.

## Testing

```bash
cargo test
cargo test symbolic  # specific test suite
cargo bench
```

## Status

Early development. Features are added incrementally. The `symbolic` dialect and its lowering passes are functional but evolving. Bug reports and contributions welcome via [issues](https://github.com/FrancoGiachetta/mathic/issues).

<div align="center">

**Built with ❤️ and 🦀 Rust**

</div>

