# Symbolic Passes

## Transform: `symbolic-extract-eval`

Extracts each `symbolic.eval` into a private function, deduplicating
identical expression trees. This allows the same expression to be evaluated
many times without re-building the symbolic DAG, and makes the eval logic
callable across blocks and functions. After this pass, each eval is replaced
by a `func::CallOp`.

**Before:**

```mlir
func.func @main() -> i32 {
  %val = arith.constant 10 : i32
  %x = symbolic.sym "x" : !symbolic.expr<i32, isSigned = true>
  %xx = symbolic.mul %x, %x : (!symbolic.expr<i32, isSigned = true>, !symbolic.expr<i32, isSigned = true>) -> !symbolic.expr<i32, isSigned = true>
  %r = symbolic.eval %xx, ["x"], %val : (!symbolic.expr<i32, isSigned = true>, i32) -> i32
  return %r : i32
}
```

**After:**

```mlir
func.func private @__eval_op_<hash>(%arg0: i32) -> !symbolic.expr<i32, isSigned = true> {
  %0 = symbolic.mul %arg0, %arg0 : (i32, i32) -> !symbolic.expr<i32, isSigned = true>
  return %0 : !symbolic.expr<i32, isSigned = true>
}

func.func @main() -> i32 {
  %val = arith.constant 10 : i32
  %0 = call @__eval_op_<hash>(%val) : (i32) -> !symbolic.expr<i32, isSigned = true>
  %1 = unrealized_conversion_cast %0 : !symbolic.expr<i32, isSigned = true> to i32
  return %1 : i32
}
```

### How it works

1. **Collect symbols**: The `symbolic.eval` is passed to arguments: `syms` and
   `values` which represent the symbols to evaluate with the corresponding
   values. `syms` is used to match the expression's `symbolic.sym` nodes based
   on the symbol's name, then its pre-mapped to the corresponding value.
   Premapping allows to avoid duplicating expressions.
2. **Collect free variables**: Walks the expression tree to find external
   values that cannot be cloned (e.g. `LLVM::LoadOp` and block arguments).
   These become extra trailing arguments to the extracted function.
3. **Clone expression**: Uses `IRMapping` + `OpBuilder::clone()` to clone
   the expression DAG into the new function body, mapping free variables
   to the corresponding block arguments.
4. **Deduplicate**: The function is named with a hash of the expression
   tree, so identical expressions reuse the same function rather than
   creating duplicates.

## Conversion: `symbolic-to-arith`

Lowers the `symbolic` dialect entirely to `arith` + `func`. The type
`!symbolic.expr` is replaced by its inner type (e.g. `expr<i32>` for `i32`).

**After both passes:**

```mlir
func.func private @__eval_op_<hash>(%arg0: i32) -> i32 {
  %0 = arith.muli %arg0, %arg0 : i32
  return %0 : i32
}

func.func @main() -> i32 {
  %val = arith.constant 10 : i32
  %0 = call @__eval_op_<hash>(%val) : (i32) -> i32
  return %0 : i32
}
```

### How it works

Uses MLIR's `DialectConversion` framework:

1. **Type conversion**: `!symbolic.expr<T>` is replaced by `T`.
2. **Operation conversion**: Each symbolic op has a pattern that rewrites it
   to the corresponding `arith` operation (`add` → `addi`, `sub` → `subi`,
   `mul` → `muli`).
3. **Division**: `symbolic.div` lowers to `arith.divsi` or `arith.divui`
   depending on the `isSigned` flag of the expression's result type.
4. **`symbolic.sym`** is replaced by the corresponding block argument (one per
   symbol) — the symbolic variable name is discarded since the expression
   tree has already been specialized for that variable during
   `symbolic-extract-eval`.
