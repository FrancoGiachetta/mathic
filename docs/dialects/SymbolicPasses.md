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
  %x = symbolic.sym "x" : !symbolic.expr
  %xx = symbolic.mul %x, %x : !symbolic.expr
  %r = symbolic.eval %xx, "x", %val : i32 -> i32
  return %r : i32
}
```

**After:**

```mlir
func.func private @__eval_op_<hash>(%arg0: i32) -> !symbolic.expr {
  %0 = symbolic.sym "x" : !symbolic.expr
  %1 = symbolic.mul %0, %0 : !symbolic.expr
  return %1 : !symbolic.expr
}

func.func @main() -> i32 {
  %val = arith.constant 10 : i32
  %0 = call @__eval_op_<hash>(%val) : (i32) -> !symbolic.expr
  %1 = unrealized_conversion_cast %0 : !symbolic.expr to i32
  return %1 : i32
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
`!symbolic.expr` is replaced by its inner type (e.g. `expr<i32>` for `i32`).

| Symbolic Op | Lowered To |
|-------------|-----------|
| `symbolic.add` | `arith.addi` |
| `symbolic.sub` | `arith.subi` |
| `symbolic.mul` | `arith.muli` |
| `symbolic.div` | `arith.divsi` / `arith.divui` |
| `symbolic.sym` | The function's single block argument |
| `symbolic.eval` | Should be handled by `symbolic-extract-eval` first |

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
   `mul` → `muli`, `div` → `divsi`/`divui` depending on signedness).
3. **`symbolic.sym`** is replaced by the extracted function's single block
   argument — the symbolic variable name is discarded since the expression
   tree has already been specialized for that variable during
   `symbolic-extract-eval`.
