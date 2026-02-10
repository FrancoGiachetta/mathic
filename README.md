<div align="center">

# 🧮 Mathic

*A programming language with builtin symbolic algebra capabilities, powered by LLVM/MLIR*

---

Mathic is a modern programming language designed for symbolic algebra and mathematical computation, leveraging the power of LLVM/MLIR for efficient code generation.

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
- 🚧 **Struct declarations** 
- 🚧 **Variable declarations** 
- 🚧 **Symbolic declarations** 

#### Control Flow
- 🚧 **For loops** 
- 🚧 **While loops** 
- 🚧 **If statements** 
- 🚧 **Return statements** 

#### Expressions
- 🚧 **Primary expressions** (identifiers, numbers, strings, booleans)
- 🚧 **Assignment expressions**
- 🚧 **Arithmetic operations** (+, -, *, /)
- 🚧 **Comparison operations** (==, !=, >, >=, <, <=)
- 🚧 **Logical operations** (and, or)
- 🚧 **Unary operations** (!, -)
- 🚧 **Function calls**

### ⚙️ Code Generation Infrastructure

#### Backend Components
- ✅ **MLIR context and module setup**
- ✅ **Dialect registry configuration**
- 🚧 **AST-to-MLIR conversion** (in progress)
- 🚧 **Function compilation** (planned)
- 🚧 **Expression compilation** (planned)

---

<div align="center">

**Built with ❤️ and 🦀 Rust**

</div>
