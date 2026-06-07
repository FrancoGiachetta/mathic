# Dialects

## Overview

The `symbolic` dialect is a custom MLIR dialect for representing symbolic
algebraic expressions as a dataflow DAG. Expressions are built from named
symbolic variables (`symbolic.sym`) and arithmetic operations (`add`, `sub`,
`mul`, `div`). A `symbolic.eval` operation substitutes a variable with a
concrete floating-point value.

The dialect is lowered to standard MLIR dialects (`arith`, `func`) through
a two-phase pipeline: extract evaluation functions, then convert operations.

---

## Type System

| MLIR Type | Mnemonic | Description |
|-----------|----------|-------------|
| `!symbolic.expr` | `expr` | A symbolic expression handle |

---

> **Note:** This dialect is still in progress. More transforms for expression
> simplification may be added in the future, possibly using e-graphs (equality
> saturation) for rewriting and canonicalization.

---

## Operations

| Operation | Description |
|-----------|-------------|
| `symbolic.sym` | Introduce a named symbolic variable |
| `symbolic.add` | Addition of two symbolic expressions |
| `symbolic.sub` | Subtraction of two symbolic expressions |
| `symbolic.mul` | Multiplication of two symbolic expressions |
| `symbolic.div` | Division of two symbolic expressions |
| `symbolic.eval` | Evaluate expression substituting a variable with f64 |

### `symbolic.sym`

Introduces a symbolic variable with a name string:

```
%0 = symbolic.sym "x" : !symbolic.expr
%1 = symbolic.sym "y" : !symbolic.expr
```

### `symbolic.add` / `sub` / `mul` / `div`

Binary arithmetic on symbolic expressions:

```
%r = symbolic.add %lhs, %rhs : !symbolic.expr
%r = symbolic.sub %lhs, %rhs : !symbolic.expr
%r = symbolic.mul %lhs, %rhs : !symbolic.expr
%r = symbolic.div %lhs, %rhs : !symbolic.expr
```

### `symbolic.eval`

Evaluates a symbolic expression by substituting a named variable with a
concrete `f64` value:

```
%result = symbolic.eval %expr, "x", %value : f64 -> f64
```

### Example

MLIR IR for `x * x + x`:

```mlir
%x = symbolic.sym "x" : !symbolic.expr
%xx = symbolic.mul %x, %x : !symbolic.expr
%r = symbolic.add %xx, %x : !symbolic.expr
```

---

## Transform: `symbolic-extract-eval`

Extracts each `symbolic.eval` into a private function, deduplicating
identical expression trees. This allows the same expression to be evaluated
many times without re-building the symbolic DAG, and makes the eval logic
callable across blocks and functions. After this pass, each eval is replaced
by a `func::CallOp`.

**Before:**

```mlir
func.func @main(%val: f64) -> f64 {
  %x = symbolic.sym "x" : !symbolic.expr
  %xx = symbolic.mul %x, %x : !symbolic.expr
  %r = symbolic.eval %xx, "x", %val : f64 -> f64
  return %r : f64
}
```

**After:**

```mlir
func.func private @__eval_op_<hash>(%arg0: f64) -> !symbolic.expr {
  %0 = symbolic.sym "x" : !symbolic.expr
  %1 = symbolic.mul %0, %0 : !symbolic.expr
  return %1 : !symbolic.expr
}

func.func @main(%val: f64) -> f64 {
  %0 = call @__eval_op_<hash>(%val) : (f64) -> !symbolic.expr
  %1 = unrealized_conversion_cast %0 : !symbolic.expr to f64
  return %1 : f64
}
```

---

## Conversion: `symbolic-to-arith`

Lowers the `symbolic` dialect entirely to `arith` + `func`. The type
`!symbolic.expr` becomes `f64`.

| Symbolic Op | Lowered To |
|-------------|-----------|
| `symbolic.add` | `arith.addf` |
| `symbolic.sub` | `arith.subf` |
| `symbolic.mul` | `arith.mulf` |
| `symbolic.div` | `arith.divf` |
| `symbolic.sym` | The function's single block argument |
| `symbolic.eval` | Should be handled by `symbolic-extract-eval` first |

**After both passes:**

```mlir
func.func private @__eval_op_<hash>(%arg0: f64) -> f64 {
  %0 = arith.mulf %arg0, %arg0 : f64
  return %0 : f64
}

func.func @main(%val: f64) -> f64 {
  %0 = call @__eval_op_<hash>(%val) : (f64) -> f64
  return %0 : f64
}
```
