# Known Issues and TODOs

## Known Issues

### Parser: Chained Function Calls Not Supported

The current implementation only supports simple function calls like `foo()` or `foo(a, b)`. 
Chained calls like `a()(b)(c)` or higher-order function calls like `getFn()()` will fail.

**Current AST:**
```rust
Call {
    calle: String,  // Only accepts identifier names
    args: Vec<ExprStmt>,
}
```

**Problem Code:**
```rust
if let ExprStmt::Primary(PrimaryExpr::Ident(calle)) = expr {
    expr = ExprStmt::Call { calle, args };
} else {
    return Err(ParseError::UnexpectedToken(
        "Expected identifier for function call".into(),
    ));
}
```

After the first call, `expr` becomes `Call { ... }`, which doesn't match `Primary(Ident(_))`, 
causing the second `()` to fail.

**Example that fails:**
```mathic
df main() {
    return a()(b)(c);  // ERROR: Expected identifier for function call
}
```

**Fix Required:**
1. Change AST `Call` to accept any expression as callee.
   ```rust
   Call {
       callee: Box<ExprStmt>,
       args: Vec<ExprStmt>,
   }
   ```

2. Update parser.
   ```rust
   expr = ExprStmt::Call {
       callee: Box::new(expr),
       args,
   };
   ```

3. Update codegen to handle non-identifier callees (would need to compile the callee expression first).

---

### Codegen: Return Statements in Control Flow Blocks

Return statements inside `if`, `for`, and `while` blocks are not supported. The issue stems from how MLIR regions and the `func` dialect work:

**Problem:**
- Control flow statements (`if`, `for`, `while`) use MLIR regions with new blocks
- The `func.return` operation requires its parent operation to be `func.func`
- When a return is inside a nested block (within a region), it's not a direct child of `func.func`
- Current codegen doesn't handle this parent operation requirement

**Example that fails:**
```mathic
df main() {
    if (condition) {
        return 42;  // ERROR: func.return not inside func.func
    }
    return 0;
}
```

**Possible Solutions:**
1. Use `scf.execute_region` to isolate control flow regions and handle returns via control flow
2. Transform early returns into structured control flow by lifting returns to the function level
3. Use branch operations to jump to a single return point at function end
4. Create a custom return operation that can be lowered appropriately based on context

**Related MLIR Concepts:**
- `scf.if`, `scf.for`, `scf.while` operations with regions
- `func.return` parent operation requirement
- Block arguments and dominance

---

## TODOs

### Symbolic System (Core Feature)

Mathic is a symbolic mathematics language. Symbols represent mathematical expressions, not values.

**Symbol Declaration:**
```mathic
sym x = a + b;  // x represents the expression "a + b", not its value
```

**TODOs:**
- Parse `sym` keyword and symbol declarations.
- Symbol table for tracking symbolic bindings.
- Expression trees for symbolic representations.
- Symbol substitution and pattern matching.
- Symbolic evaluation engine (new mlir dialect).

**Features to implement:**
- Symbolic algebraic operations (expand, factor, simplify).
- Equation solving (symbolic manipulation).
- Calculus operations (derivatives, integrals).
- Pattern matching for rewrite rules.
- Pretty printing of symbolic expressions.

### Parser

#### Variable Declarations

```rust
Token::Struct | Token::Let | Token::Sym => {
    todo!()
}
```

Need to implement:
- `let x = expr;`: variable declarations (runtime values).
- `struct Foo { ... }`: struct declarations.

#### Functions' Return Type Parsing

Function return types are not parsed:
```rust
// Return type parsing should be here.
```

Grammar supports: `df ident() -> type { ... }`

#### Parameter Type Parsing

Parameter types are not parsed:
```rust
// Param's type parsing should be here.
```

Grammar supports: `df foo(x: i32, y: i32) { ... }`

### Codegen

#### Variable Allocation
- Stack allocation for local variables.
- Handle variable scoping and shadowing.

#### Control Flow
- Break and continue statements.

#### Function Calls
- Support function arguments in calls.
- Handle return values properly.
- Function pointer support (for chained calls).

#### Error Handling
- Runtime error reporting (division by zero, etc.).
- Stack traces for debugging.

---

## Future Possibilities

### Salsa (Incremental Computation Framework)

Salsa provides incremental recomputation for multi-phase compilers. Each phase (parse, type-check, codegen) becomes a cached query that only re-executes when its inputs change.

**Use case here:** Enable incremental compilation and LSP support (go-to-def, autocomplete) by caching AST, types, and IR between compiles.

**Potential implementations:**
- Incremental recompilation (only re-parse changed files).
- Persistent compilation cache across runs.
- Parallel compilation phases.
