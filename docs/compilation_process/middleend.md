# MATHIR: Mathic's Intermediate Representation

## Why do we need MATHIR?

MATHIR exists because generating MLIR directly from the AST is unnecessarily complex. MATHIR is a simpler IR — no SSA, no dialects, no basic block terminators — that sits between the AST and MLIR, making the lowering easier to reason about and debug.
Mathic uses MLIR as its main backend infrastructure, which uses the same representation as LLVM, based on the concept of basic blocks. Due to this, we cannot simply lower the AST directly to MLIR — we wouldn't be able to do things like return values inside if statements (since we have no way of tracking which block we're currently in). Loops would also be difficult to implement due to the SSA nature of MLIR.

## How is MATHIR Organized

MATHIR is an LLVM/MLIR-like IR, where the code is organized using blocks. It's not SSA (variables can be reassigned directly), keeping the IR closer to the source structure and easier to debug.

### Basic Blocks

MATHIR is organized as a set of sequentially numbered blocks. Each basic block holds a sequence of non-branching instructions to be executed, followed by a branching instruction (commonly called [terminator](#terminators)) that marks the end of a block and moves the control flow to the next one.

In the [example above](#example), `block0` initializes the variables and branches to `block1`. `block1` has a single `cond_br` terminator that decides whether to enter the loop body (`block2`) or exit (`block3`).

Basic blocks are represented [here](../../src/lowering/ir/basic_block.rs:15)

### Instructions

Instructions are a unit of operation. There are two kinds:

* `l-value` instructions: instructions with side-effects in the state of the program.
* `r-value` instructions: instructions which do not generate any change in the state of the program, commonly related to expressions.

Both are represented [here](../../src/lowering/ir/instruction.rs)

Looking at the [example](#example), `let %0 = 1` is an l-value instruction (it declares a local), while `%0 * %2` is an r-value instruction that computes a multiplication.

### Terminators

Terminators are a special type of instructions which affect the control flow of the program. Every basic block **must** have one and only one terminator since they mark the end of the block, moving the control flow to the next block or returning a value (if it represents the end of the program).

In the [example](#example), `block0` ends with `br block1 []` (unconditional branch), `block1` ends with `cond_br (...)` (conditional branch), and `block3` ends with `return %0`.

Terminators are represented [here](../../src/lowering/ir/basic_block.rs)

### Functions

A Mathic program is composed of functions — no code can live outside a function. The entrypoint of a program is the `main` function.
MATHIR follows the same rule: blocks cannot live outside a function. A function holds a set of sequentially numbered blocks, zero or more parameters, and an optional return type.

Functions are represented [here](../../src/lowering/ir/function.rs)

### Types

Mathic is statically typed, meaning every variable **must have** an associated type.

### Example

Here's a quick preview of what MATHIR looks like.
Mathic program:

```rust
df main() i32 {
    let result: i32 = 1;
    let n: i32 = 5;
    let i: i32 = 1;
    while i <= n {
        result = result * i;
        i = i + 1;
    }
    return result;
}
```

MATHIR output:

```
df main() -> i64 {
    block0: {
        let %0 = 1
        let %1 = 5
        let %2 = 1
        br block1 []
    }
    block1: {
        cond_br (%2 <= %1) then block2 [] else block3 []
    }
    block2: {
        %0 = %0 * %2
        %2 = %2 + 1
        br block1 []
    }
    block3: {
        return %0
    }
}
```

## Lowering the AST
