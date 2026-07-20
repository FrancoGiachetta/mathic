# MATHIR: Mathic's Intermediate Representation

## Why do we need MATHIR?

MATHIR exists because generating MLIR directly from the AST is unnecessarily complex. MATHIR is a simpler IR — no SSA, no dialects, no basic block terminators — that sits between the AST and MLIR, making the lowering easier to reason about and debug.
Mathic uses MLIR as its main backend infrastructure, which uses the same representation as LLVM, based on the concept of basic blocks. Due to this, we cannot simply lower the AST directly to MLIR — we wouldn't be able to do things like return values inside if statements (since we have no way of tracking which block we're currently in). Loops would also be difficult to implement due to the SSA nature of MLIR.

## How is MATHIR Organized

MATHIR is an LLVM/MLIR-like IR, where the code is organized using blocks. It's not SSA (variables can be reassigned directly), keeping the IR closer to the source structure and easier to debug.

### Basic Blocks

MATHIR is organized as a set of sequentially numbered blocks. Each basic block holds a sequence of non-branching instructions to be executed, followed by a branching instruction (commonly called [terminator](#terminators)) that marks the end of a block and moves the control flow to the next one.

In the [example above](#example), `block0` initializes the variables and branches to `block1`. `block1` has a single `cond_br` terminator that decides whether to enter the loop body (`block2`) or exit (`block3`).

Basic blocks are represented [here](../../src/lowering/ir/basic_block.rs#15)

### Instructions

Instructions are a unit of operation. There are two kinds:

* `l-value` instructions: instructions with side-effects in the state of the program.
* `r-value` instructions: instructions which do not generate any change in the state of the program, commonly related to expressions.

Both are represented [here](../../src/lowering/ir/instruction.rs)

Looking at the [example](#example), `let %0 = 1` is an l-value instruction (it declares a local), while `%0 * %2` is an r-value instruction that computes a multiplication.

### Terminators

Terminators are a special type of instructions which affect the control flow of the program. Every basic block **must** have one and only one terminator since they mark its end, transferring the control flow to the next block or returning from a function (if it represents the end of the program).

In the [example](#example), `block0` ends with `br block1 []` (unconditional branch), `block1` ends with `cond_br (...)` (conditional branch), and `block3` ends with `return %0`.

Terminators are represented [here](../../src/lowering/ir/basic_block.rs#42)

### Functions

A Mathic program is composed of functions — no code can live outside a function. The entrypoint of a program is the `main` function.
MATHIR follows the same rule: blocks cannot live outside a function. A function holds a set of sequentially numbered blocks, zero or more parameters, and an optional return type.

Functions are represented [here](../../src/lowering/ir/function.rs#38)

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

The lowering process begins with the call of the `lower_program` [here](../../src/lowering.rs#33). This function does two main things:

1. Creates the `IRBuilder`.
2. Loops the top level items to begin lowering.

> This is an auxiliary structure to avoid having ownership issues during the lowering. It holds what's necessary to create the IR.

Structurally, a Mathic program is composed of either function or struct declarations (top level items). The top level items of the AST are iterated twice:

1. The first one is to cache the declarations by storing the AST sub-tree. This allows to reference items before they are declared. For example function calls before their declaration. To track these declarations, the IRBuilder has a [declaration table](../../src/lowering/ir/symbols.rs#22).
2. The second one is to lower them.

### Lowering Functions

So, before we can lower statements we need to lower what will hold them, functions. There can be top level functions and local functions (a function inside another). For this reason, their lowering is handled by different functions: `lower_top_level_function` [here](../../src/lowering.rs#63) and `lower_inner_function` [here](../../src/lowering/ast_lowering/declaration.rs#105). Both do the same thing, they only differ in their scope. 
To lower a function, a [FunctionBuilder](#functionbuilder) is constructed from its `name`, `return type` and `params`. Next we loop over the functions [statements](#lowering-statements) to lower them.

#### FunctionBuilder

For the same reason we have the `IRBuilder`, we have the `FunctionBuilder` [here](../../src/lowering/ir/function.rs#74). It holds a mutable reference to the `IRBuilder` to make it easy to make a global change if ever needed. It also has a declaration table to cache declarations and a [symbol table](#symbol-table) from where it can take any kind of symbol (from locals to functions, user defined types and ADTs).

### Lowering ADTs

ADTs (Abstract Data Structures) are the other top level item apart from functions. For now, they represent structs, but in the future they could also be an enum.

### Lowering Statements

There are three types of statements:

#### Declaration

We can declare a function, an ADT or a local. Their lowering entrypoints can be found [here](../../src/lowering/ast_lowering/declaration.rs). 
Declaring something means declaraing a symbol, and for that reason we need a symbol table.

##### Symbol Table

It allows to track any symbol declared through the program (either a local, function, ADT). It is defined [here](../../src/lowering/ir/symbols.rs#88). There's a symbol table per function to keep track of any local symbols declared.


#### Control Flow

#### Expression

### Lowering Expressions

