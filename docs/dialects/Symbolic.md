# Symbolic Dialect

## Overview

The `symbolic` dialect is a custom MLIR dialect for representing symbolic
algebraic expressions as a dataflow DAG. Expressions are built from named
symbolic variables (`symbolic.sym`) and arithmetic operations (`add`, `sub`,
`mul`, `div`). A `symbolic.eval` operation substitutes a variable with a
concrete floating-point value.

The dialect is lowered to standard MLIR dialects (`arith`, `func`) through a
two-phase pipeline: extract evaluation functions, then convert operations
(see [SymbolicPasses.md](SymbolicPasses.md)).

## Type System

| MLIR Type | Mnemonic | Description |
|-----------|----------|-------------|
| `!symbolic.expr` | `expr` | A symbolic expression handle |

## Operations

### `symbolic.sym`

Introduces a symbolic variable with a name string:

```mlir
%0 = symbolic.sym "x" : !symbolic.expr
%1 = symbolic.sym "y" : !symbolic.expr
```

### `symbolic.add` / `sub` / `mul` / `div`

Binary arithmetic. Both operands accept either symbolic expressions or
concrete integers:

```mlir
%r = symbolic.add %lhs, %rhs : !symbolic.expr
%r = symbolic.sub %lhs, %rhs : !symbolic.expr
%r = symbolic.mul %lhs, %rhs : !symbolic.expr
%r = symbolic.div %lhs, %rhs : !symbolic.expr
```

### `symbolic.eval`

Evaluates a symbolic expression by substituting a named variable with a
concrete `f64` value:

```mlir
%result = symbolic.eval %expr, "x", %value : f64 -> f64
```

### Example

MLIR IR for `x * x + x`:

```mlir
%x = symbolic.sym "x" : !symbolic.expr
%xx = symbolic.mul %x, %x : !symbolic.expr
%r = symbolic.add %xx, %x : !symbolic.expr
```

## Project Structure

```
Dialects/
├── CMakeLists.txt
├── Makefile
├── include/
│   ├── Dialect/
│   │   ├── capi.h
│   │   └── Symbolic/
│   │       ├── Conversions/SymbolicToArith/
│   │       │   ├── SymbolicToArith.h
│   │       │   └── SymbolicToArith.td
│   │       ├── IR/
│   │       │   ├── SymbolicDialect.h
│   │       │   ├── SymbolicDialect.td
│   │       │   ├── SymbolicOps.h
│   │       │   ├── SymbolicOps.td
│   │       │   ├── SymbolicTypes.h
│   │       │   └── SymbolicTypes.td
│   │       └── Transforms/
│   │           ├── Passes.h
│   │           ├── Passes.td
│   │           └── SymbolicExtractEval.h
├── lib/
│   └── Dialect/
│       ├── capi.cpp
│       └── Symbolic/
│           ├── Conversions/SymbolicToArith/
│           │   └── SymbolicToArith.cpp
│           ├── IR/
│           │   ├── SymbolicDialect.cpp
│           │   ├── SymbolicOps.cpp
│           │   └── SymbolicTypes.cpp
│           └── Transforms/
│               └── SymbolicExtractEval.cpp
├── tools/
│   └── dialect-driver.cpp
└── tests/
    ├── CMakeLists.txt
    ├── Dialect/
    │   └── Symbolic/
    │       └── (lit tests)
    ├── lit.cfg.py
    └── lit.site.cfg.py.in
```
