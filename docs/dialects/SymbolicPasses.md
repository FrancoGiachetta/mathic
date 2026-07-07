# Symbolic Passes

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

### How it works

1. **Collect free variables**: Walks the expression tree to find external
   values that cannot be cloned (e.g. `LLVM::LoadOp`, block arguments).
   These become extra arguments to the extracted function.
2. **Clone expression**: Uses `IRMapping` + `OpBuilder::clone()` to clone
   the expression DAG into the new function body, mapping free variables
   to the corresponding block arguments.
3. **Deduplicate**: The function is named with a hash of the expression
   tree, so identical expressions reuse the same function rather than
   creating duplicates.

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

### How it works

Uses a `BINARY_OP_CONVERTER(SYM_OP, ARITH_OP)` macro that replaces each
`symbolic.{add,sub,mul,div}` with the corresponding
`arith.{addf,subf,mulf,divf}`. The `symbolic.sym` operation is replaced by
the function's single block argument — the symbolic variable name is
discarded at this stage since the expression tree has already been
specialized for that variable during `symbolic-extract-eval`.
