# Assignment as Expression Refactoring Plan

## Overview
Move assignment from being a statement type to being an expression type. This allows assignment to be used anywhere an expression is expected (e.g., `return (x = 4)`).

## Grammar Changes

### 1. grammar.txt
**Changes:**
- Remove `<assignment_stmt>` from `<statement>` alternatives
- Add `<expr_stmt>` to `<statement>` alternatives
- Update `<expr>` to start with `<assignment>`
- Add `<assignment>` production
- Add `<expr_stmt>` production
- Remove `<assignment_stmt>` production

```ebnf
<statement> := <declaration>
             | <for_stmt>
             | <while_stmt>
             | <if_stmt>
             | <return_stmt>
             | <expr_stmt>        // NEW
             | <block>

<expr>       := <assignment>       // CHANGED: was <logic_or>
<assignment> := <ident> "=" <logic_or> | <logic_or>  // NEW
<expr_stmt>  := <expr> ";"         // NEW

// REMOVED:
// <assignment_stmt> := <ident> "=" <expr> ";"
```

## AST Changes

### 2. src/parser/ast/expression.rs
**Add to `ExprStmt` enum:**
```rust
Assign {
    name: String,
    value: Box<ExprStmt>,
},
```

### 3. src/parser/ast/statement.rs
**Changes:**
- Replace `Stmt::Assign { name, value }` with `Stmt::Expr(ExprStmt)`
- Remove `ExprStmt` import if no longer needed

```rust
pub enum Stmt {
    Decl(DeclStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Return(ExprStmt),
    Expr(ExprStmt),  // NEW - replaces Assign
}
```

## Parser Changes

### 4. src/parser/parsing/expression.rs
**Add method:**
```rust
fn parse_assignment(&self) -> ParserResult<ExprStmt> {
    // Check if current token is Ident and next is Eq
    // If so: parse ident, consume Eq, parse logic_or, return Assign
    // Otherwise: just parse logic_or
}
```

**Update `parse_expr()`:**
```rust
pub fn parse_expr(&self) -> ParserResult<ExprStmt> {
    self.parse_assignment()  // CHANGED: was parse_logic_or()
}
```

### 5. src/parser/parsing/statement.rs
**Changes in `parse_stmt()`:**
- Remove `Token::Ident` case with assignment check
- Remove call to `parse_assignment_stmt()`
- Handle expression statements by checking for expression start tokens
- New case for expression statements

```rust
Token::Ident => {
    // Parse as expression statement
    let expr = self.parse_expr()?;
    self.consume_token(Token::Semicolon)?;
    Ok(Stmt::Expr(expr))
}
// OR better - handle all expression starts generically
```

**Remove method:**
```rust
fn parse_assignment_stmt(&self) -> ParserResult<Stmt>
```

**Add method:**
```rust
fn parse_expr_stmt(&self) -> ParserResult<Stmt> {
    let expr = self.parse_expr()?;
    self.consume_token(Token::Semicolon)?;
    Ok(Stmt::Expr(expr))
}
```

## Codegen Changes

### 6. src/codegen/expression.rs
**Add method:**
```rust
fn compile_assign<'ctx, 'func>(
    &'func self,
    block: &'func Block<'ctx>,
    name: &str,
    value: &ExprStmt,
) -> Result<Value<'ctx, 'func>, CodegenError>
where
    'func: 'ctx,
{
    // Compile the value expression
    // Store to symbol
    // Return the stored value
}
```

**Update `compile_expression()`:**
```rust
match expr {
    // ... existing cases ...
    ExprStmt::Assign { name, value } => self.compile_assign(block, name, value),
}
```

### 7. src/codegen/statement.rs
**Update `compile_statement()`:**
```rust
match stmt {
    // ... existing cases ...
    Stmt::Assign { .. } => // REMOVE THIS CASE
    Stmt::Expr(expr) => {
        // Compile expression, discard result
        let _ = self.compile_expression(block, expr)?;
        Ok(())
    }
}
```

**Remove method:**
```rust
fn compile_assignment(...)
```

## Testing Considerations

After changes, verify:
1. `x = 5;` still works as statement
2. `return x = 5;` works (returns 5)
3. `a = b = 5` parses correctly (if right-recursive grammar)
4. All existing tests pass
5. Assignment in expressions works in all contexts

## Files to Modify Summary

1. grammar.txt
2. src/parser/ast/expression.rs
3. src/parser/ast/statement.rs
4. src/parser/parsing/expression.rs
5. src/parser/parsing/statement.rs
6. src/codegen/expression.rs
7. src/codegen/statement.rs

## Next Steps

Proceed with implementation in the order listed above, testing after each major change.
