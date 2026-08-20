# Symbolic Dialect

## Overview

The `symbolic` dialect is a custom MLIR dialect for representing symbolic
algebraic expressions as a dataflow DAG. Expressions are built from named
symbolic variables (`symbolic.sym`) and arithmetic operations (`add`, `sub`,
`mul`, `div`). A `symbolic.eval` operation substitutes one or more variables
with concrete values (e.g. `i32`).

The dialect is lowered to standard MLIR dialects (`arith`, `func`) through a
two-phase pipeline: extract evaluation functions, then convert operations
(see [SymbolicPasses.md](SymbolicPasses.md)).

## Type System

| MLIR Type | Mnemonic | Description |
|-----------|----------|-------------|
| `!symbolic.expr<innerType, isSigned>` | `expr` | A symbolic expression handle with an inner integer type and a signedness flag (e.g. `!symbolic.expr<i32, isSigned = true>` for `expr<i32>`) |

## Operations

### `symbolic.sym`

Introduces a symbolic variable with a name string and an expression type:

```mlir
%0 = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
%1 = symbolic.sym "y" : !symbolic.expr<i32, isSigned = true>
```

### `symbolic.add` / `sub` / `mul` / `div`

Binary arithmetic. Both operands accept either symbolic expressions or
concrete integers, and produce a symbolic expression:

```mlir
%r = symbolic.add %lhs, %rhs : (!symbolic.expr<i32, isSigned = true>, i32) -> !symbolic.expr<i32, isSigned = true>
%r = symbolic.sub %lhs, %rhs : (i32, i32) -> !symbolic.expr<i32, isSigned = true>
%r = symbolic.mul %lhs, %rhs : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
%r = symbolic.div %lhs, %rhs : (i32, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
```

### `symbolic.eval`

Evaluates a symbolic expression by substituting one or more named variables
with concrete values. Symbols are given as a string array with one value per
symbol:

```mlir
%result = symbolic.eval %expr, ["x"], %value : (!symbolic.expr<i32, isSigned = true>, i32) -> i32
%result = symbolic.eval %expr, ["x", "y"], %vx, %vy : (!symbolic.expr<i32, isSigned = true>, i32, i32) -> i32
```

### Example

MLIR IR for `x * x + x`:

```mlir
%x = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
%xx = symbolic.mul %x, %x : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
%r = symbolic.add %xx, %x : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
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
