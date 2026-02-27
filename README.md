<div align="center">

# 🧮 Mathic

*A programming language with builtin symbolic algebra capabilities, powered by LLVM/MLIR*

</div>

## 🔧 Dependencies

- **Rust** 1.93 or higher
- **LLVM/MLIR** 21.x.x 

### LLVM/MLIR Installation

There are many ways of installing LLVM. Choose the one that fits your needs.

#### MacOS (Homebrew)

```bash
brew install llvm@21
```

After installation, set the environment variables:

```sh
export LLVM_SYS_211_PREFIX=$(brew --prefix)/opt/llvm@21
export MLIR_SYS_210_PREFIX=$(brew --prefix)/opt/llvm@21
export TABLEGEN_210_PREFIX=$(brew --prefix)/opt/llvm@21
```

#### Building from Source

> ⚠️ Note: Building LLVM from source requires at least 6GB of RAM and ~20GB of
disk space. Ensure these requirements are met, as the build process is likely
to fail otherwise.

1. **Clone LLVM Project**
```bash
git clone https://github.com/llvm/llvm-project.git
cd llvm-project
git checkout llvmorg-21.1.7
mkdir build && cd build
```

2. **Configure Build**
```bash
cmake -G Ninja ../llvm \
   -DLLVM_ENABLE_PROJECTS="mlir" \
   -DCMAKE_BUILD_TYPE=RelWithDebInfo \
   -DLLVM_ENABLE_ASSERTIONS=On \
   -DLLVM_USE_LINKER=mold \
   -DLLVM_BUILD_LLVM_DYLIB=On \
   -DLLVM_LINK_LLVM_DYLIB=On \
   -DMLIR_BUILD_MLIR_C_DYLIB=On \
   -DLLVM_TARGETS_TO_BUILD=host \
   -DCMAKE_INSTALL_PREFIX=/opt/llvm-21
```

3. **Build and Install**
```bash
ninja install
```

You'll need to export the required environment variables as well:

```sh
export LLVM_SYS_211_PREFIX=<path-to-install-prefix>
export MLIR_SYS_210_PREFIX=<path-to-install-prefix>
export TABLEGEN_210_PREFIX=<path-to-install-prefix>
```

> If you used to command above, the prefix will be `/opt/llvm-21`

## Usage

You can run a program using this command:

```bash
cargo --bin euler -- <path-to-file>.mth 
```

## 📖 Current Status

> ⚠️ **Note**: This project is in early development. Features are being added incrementally.

---

For more details, see the [docs](./docs/README.md).

<div align="center">

**Built with ❤️ and 🦀 Rust**

</div>
