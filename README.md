# Overview

Mathic is a programming language with symbolic algebra capabilities, using LLVM/MLIR for code generation.

# Dependencies

## LLVM/MLIR

LLVM can be install in many different ways, the common one is by building it from the source code.

1. Clone llvm-project repository.
2. Create a build/ directory in it.
3. Inside build/, run this command to build llvm/mlir:

```shell
cmake -G Ninja ../llvm \
    -DLLVM_ENABLE_PROJECTS="mlir" \
    -DCMAKE_BUILD_TYPE=RelWithDebInfo \
    -DLLVM_ENABLE_ASSERTIONS=On \
    -DLLVM_USE_LINKER=mold \
    -DLLVM_BUILD_LLVM_DYLIB=On \
    -DLLVM_LINK_LLVM_DYLIB=On \
    -DMLIR_BUILD_MLIR_C_DYLIB=On \
    -DLLVM_TARGETS_TO_BUILD=host \
    -DLLVM_PARALLEL_COMPILE_JOBS=4 \
    -DLLVM_PARALLEL_LINK_JOBS=2 \
    -DCMAKE_INSTALL_PREFIX=<llvm-install-prefix>
```

4. Finally, install the build:

```
ninja install
```

## Current State

This project is currently in early development.

### Language Features

**Statements**
- **Declarations**
  - 󰡢 Function declarations 
  - ❌ Struct declarations 
  - ❌ Variable declarations
  - ❌ Symbolic declarations

- **Control Flow**
  - ❌ For loops
  - ❌ While loops  
  - ❌ If statements
  - ❌ Return statements

**Expressions**
- ❌ Primary expressions (identifiers, numbers, strings, booleans)
- ❌ Assignment expressions
- ❌ Arithmetic operations (+, -, *, /)
- ❌ Comparison operations (==, !=, >, >=, <, <=)
- ❌ Logical operations (and, or)
- ❌ Unary operations (!, -)
- ❌ Function calls

### Code Generation

**Infrastructure:**
- ✅ MLIR context and module setup
- ✅ Dialect registry configuration
- ❌ AST-to-MLIR conversion
- ❌ Function compilation
- ❌ Expression compilation
