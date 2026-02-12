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

## Error Reporting

### Better Error Messages

**Current Issues:**
- Parse errors lack line/column information
- Errors don't show source context or spans
- No syntax highlighting in error output
- Limited error recovery

**Solution:** Add `ariadne` crate for beautiful compiler error messages

**Examples of desired output:**
```
error[E001]: Type mismatch
  ┌─ test.mth:5:10
  │
5 │     return x + 1;
  │          -   ^ expected `i32`, found `f64`
  │          │
  │          this expression has type `f64`
```

**Implementation needed:**
1. Add span tracking to lexer (already has spans)
2. Replace `ParseError` with ariadne-compatible types
3. Create `Report` instances with proper labels
4. Update all error sites to include span information


