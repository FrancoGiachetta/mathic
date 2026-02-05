# Overview

Mathic is a mathematical compiler with symbolic algebra capabilities, using LLVM/MLIR for code generation.

# Dependencies

## LLVM/MLIR

LLVM can be install in many different ways, the uninversal one is by building from source.

1. Clone llvm-project repository.
2. Create a build/ directory in it.
3. Inside build/, run this command:

```shell
cmake -G Ninja ../llvm \
   -DLLVM_ENABLE_PROJECTS="llvm;mlir;polly" \
   -DCMAKE_BUILD_TYPE=RelWithDebInfo \
   -DLLVM_ENABLE_ASSERTIONS=On \
   -DLLVM_USE_LINKER=mold \
   -DLLVM_LINK_LLVM_DYLIB=On \
   -DMLIR_LINK_MLIR_DYLIB=On \
   -DCMAKE_INSTALL_PREFIX=<path-to-install-llvm>
```

4. The run:

```
ninja install
```

## Current State

This project is currently in early development.

### Language Features

**Statements**
- **Declarations**
  - ❌ Function declarations 
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
