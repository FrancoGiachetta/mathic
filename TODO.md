# Known Issues and TODOs

## Parser Issues

### Chained Function Calls Not Supported

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
1. Change AST `Call` to accept any expression as callee:
   ```rust
   Call {
       callee: Box<ExprStmt>,
       args: Vec<ExprStmt>,
   }
   ```

2. Update parser:
   ```rust
   expr = ExprStmt::Call {
       callee: Box::new(expr),
       args,
   };
   ```

3. Update codegen to handle non-identifier callees (would need to compile the callee expression first)



---

## Codegen Issues

---

## Parser TODOs

### Variable Declarations

```rust
Token::Struct | Token::Let | Token::Sym => {
    todo!()
}
```

Need to implement:
- `let x = expr;` - variable declarations
- `sym x = y;` - symbolic declarations
- `struct Foo { ... }` - struct declarations

### Return Type Parsing

Function return types are not parsed:
```rust
// Return type parsing should be here.
```

Grammar supports: `df ident() -> type { ... }`

### Parameter Type Parsing

Parameter types are not parsed:
```rust
// Param's type parsing should be here.
```

Grammar supports: `df foo(x: i32, y: i32) { ... }`

---

## Future Possibilities

### Salsa (Incremental Computation Framework)

Salsa provides incremental recomputation for multi-phase compilers. Each phase (parse, type-check, codegen) becomes a cached query that only re-executes when its inputs change.

**Use case here:** Enable incremental compilation and LSP support (go-to-def, autocomplete) by caching AST, types, and IR between compiles.

**Potential implementations:**
- Incremental recompilation (only re-parse changed files)
- Persistent compilation cache across runs
- Parallel compilation phases

