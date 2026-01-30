# Overview

Mathic is a mathematical compiler with symbolic algebra capabilities, using LLVM/MLIR for code generation.

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
