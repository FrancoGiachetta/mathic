<div align="center">

# 🧮 Mathic

*A programming language with builtin symbolic algebra capabilities, powered by LLVM/MLIR*

</div>

## 🔧 Dependencies

- **Rust** 1.93 or higher
- **LLVM/MLIR** 21.x.x 

### LLVM/MLIR Installation

There are many was of installing LLVM. The most commong one it by building it from source.

1. **Clone LLVM Project**
   ```bash
   git clone https://github.com/llvm/llvm-project.git
   cd llvm-project
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

## 📖 Current Status

> ⚠️ **Note**: This project is in early development. Features are being added incrementally.

### 🏗️ Language Features

#### Statements
- ✅ **Function declarations**
- ✅ **Return statements**
- ✅ **Block statements**
- 🚧 **Variable declarations** 
- 🚧 **Struct declarations** 
- 🚧 **Symbolic declarations** 

#### Control Flow
- ✅ **If statements** (parsing)
- ✅ **While loops** (parsing)
- ✅ **For loops** (parsing)

#### Expressions
- ✅ **Primary expressions** (identifiers, numbers, booleans)
- ✅ **Arithmetic operations** (+, -, *, /)
- ✅ **Comparison operations** (==, !=, >, >=, <, <=)
- ✅ **Logical operations** (and, or)
- ✅ **Unary operations** (!, -)
- ✅ **Function calls**
- ✅ **Parenthesized expressions**
- ✅ **Operator precedence** (full precedence climbing)

### ⚙️ Code Generation Infrastructure

#### Backend Components
- ✅ **MLIR context and module setup**
- ✅ **Dialect registry configuration**
- ✅ **Expression compilation** (arithmetic, logical, comparisons)
- ✅ **Return statement compilation**
- 🚧 **Control flow codegen** (if, while, for)
- 🚧 **Statement compilation** (blocks, declarations)

---

<div align="center">

**Built with ❤️ and 🦀 Rust**

</div>
