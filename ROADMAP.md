# Mathic Roadmap

> **Note:** This is a tentative roadmap and may change as the language evolves.

## Phase 1: Fundamentals

### 1.1 Modules
- Import/export between files
- Syntax: `import std.io;` or `import "file.mth"`
- Module files with visibility rules

---

## Phase 2: Memory & Data

### 2.1. Constants
- Named constants: `const PI = 3.14159;`

### 2.3. Arrays
- Fixed-size arrays: `let arr: [i64, 5];`

---

## Phase 3: User-Defined Types

### 3.1 Enums
```mathic
enum Color {
    Red,
    Green,
    Blue,
}
```

### 3.2. Pattern Matching
- `match` expressions on enums/structs
- Exhaustiveness checking

---

## Phase 4: Generics

### 4.1. Generics
- Generic functions: `fn identity<T>(x: T) -> T`
- Generic structs: `struct Box<T> { value: T }`

### 4.2. Pointers
- Type: `ptr<T>`
- Syntax: `ptr<i64>`, `ptr<str>`
- Dereference operators: `*ptr`, `&var`
- Null pointer handling

---

## Phase 5: Standard Library

### 5.1. Std Library
- I/O: `print`, `println`, `read`, `write`
- Collections: `Vec<T>`, `Map<K, V>`, `Set<T>`, `String`
- String methods

---

## Undefined

### 1. Symbolic Algebra
- Expression AST: represent `x + 2*y`, `sin(x)^2`
- Differentiation: `derive(expr, x)`
- Simplification: `simplify(expr)`
- Substitution: `substitute(expr, x, 5)`
