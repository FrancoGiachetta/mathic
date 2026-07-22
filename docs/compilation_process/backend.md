# Mathic's Code Generation

The purpose of this section is to give a high level view of how the process of Mathic's code generation looks like, what technologies are used and some design decisions. It's not a rigurous step by step but this should be enough to be able to start reading the code and understand it.

## Gentle introduction to MLIR (Multi Level Intermediate Representation)

Mathic's code generation is based on MLIR, a compiler infrastructure that provides a modular and extensible intermediate representation to ease the construction of domain-specific compilers. To achieve this, it introduces the concept of [Dialect](#what-is-a-dialect) to allow multiple levels of abstractions in a single IR.

### How is an MLIR IR structured?

There are three core concepts:

- **Operation**: the fundamental unit of IR. An operation can be as simple as an integer addition or as complex as a function definition or a loop — the difference is whether it carries Regions.
- **Region**: a list of basic Blocks. Operations like `func.func` or `scf.for` contain Regions to define their body.
- **Block**: a list of Operations followed by a **terminator** (e.g. `func.return`, `scf.yield`). Blocks can have **block arguments**, which are SSA values scoped to that block.

These three form a recursive structure: Operations can contain Regions, Regions contain Blocks, and Blocks contain Operations.

```mlir
module {
  func.func @main() -> i32 {
    %0 = arith.constant 0 : i32
    %1 = scf.for %i = 0 to 10 step 1 iter_args(%acc = %0) -> i32 {
      %2 = arith.addi %acc, %i : i32
      scf.yield %2 : i32
    }
    return %1 : i32
  }
}
```

Here, `module` is itself an operation (the root operation), containing a single Region with one Block. Inside that Block is a `func.func` operation, which also contains a Region with one Block. That inner Block holds several operations: `arith.constant` produces `%0`, `scf.for` carries its own Region with a Block that contains `arith.addi` and `scf.yield`, and finally `func.return` consumes the result of the loop.

### What is a Dialect?

Taken directly from the [MLIR page](https://mlir.llvm.org/docs/LangRef/#dialects), a Dialect is a mechanism by which we can extend the MLIR ecosystem. They allow for the definition of new operations, types, attributes that all together model a specific domain. By allowing different dialects to coexist in the same IR, MLIR achieves the modularity we mentioned.

#### How is a Dialect made of?

Each Dialect is composed of three main components:

- **Operations**: the core instructions of the dialect. Each operation has a name, a list of inputs (operands), a list of results, and may carry attributes with compile-time information.
- **Types**: dialects can define their own type system. For example, the `arith` dialect operates on integer and float types, while other dialects may introduce domain-specific types.
- **Attributes**: compile-time metadata attached to operations, such as constant values, names, references to other IR entities, or configuration flags.

For more information on how dialects are defined and used, check the [MLIR Language Reference](https://mlir.llvm.org/docs/LangRef/) and the [Defining Dialects guide](https://mlir.llvm.org/docs/DefiningDialects/).

#### Dialect Conversion

Some dialects' operations may be on a higher level than others, this is why MLIR provides a framework to convert between different dialects. This is achieved through the [Passes](https://mlir.llvm.org/docs/PassManagement/), the basic infrastructure MLIR provides for IR transformation and optimization. Some of them are optimization passes, which transform the IR to make it more optimal. However, there's another type of passes, which make conversions between dialects.

Dialect conversion is quite useful, and necessary. In the context of Mathic, which needs to generate machine code, its target is LLVM. So, it needs dialect conversion passes that convert higher level dialects to LLVM or rather lower them to the LLVM dialect.

#### Examples of Dialects

We have [Arith](https://mlir.llvm.org/docs/Dialects/ArithOps/). `Arith` is a dialect which allows you to perform arithmetic operations with integers and floating points. Thus, it models the domain of arithmetic.

We also have the [LLVM dialect](https://mlir.llvm.org/docs/Dialects/LLVM/), which is intended to map [LLVMIR](https://llvm.org/docs/LangRef.html) into MLIR. The LLVM dialect is actually Mathic's target dialect.

## Lowering MATHIR to MLIR

### What are we using?

Since MLIR is writen in C++ and Mathic is implemented in Rust, we are using [melior crate](https://crates.io/crates/melior) to the necessary bindings. You can find a small intro to how to crate is used [here](https://edgl.dev/blog/mlir-with-rust/).

### Code Generation Flow

Thanks to MATHIR, generating MLIR is almost trivial since their structure is similar.

The code generation begins by creating an MLIR module. We create it using the `create_module` function [here](../../src/ffi.rs#45). It does a bunch of things like retrieving the [target triple](https://wiki.osdev.org/Target_Tr) and the data layout, which gives LLVM information about the platform on which the code is running, as well as alignment information and other necessary things.

So, once we have created an empty MLIR module, we need to populate it with blocks, and those blocks with operations. This is all handled by `MathicCodegen::generate_module` [here](../../src/codegen.rs#86). As with the AST lowering, we begin by compiling functions.

The compilation is structured in two main files:

- [lvalue.rs](../../src/codegen/lvalue.rs): methods in charge of compiling statements.
- [rvalue.rs](../../src/codegen/rvalue.rs): methods in charge of compiling expressions.

### Compiling a Function

The method in charge of compiling a function can be found [here](../../src/codegen/lvalue.rs#138). The first thing we do is to create a [`FunctionCtx`](#functionctx). If we had any parameters, we need to prepare the block arguments for the entry block of the function.
Before we begin compiling the actual body of a function, we need to compile the parameters. A parameter is a [Local](#locals), so they will be stored in the stack. After that, we compile the function body, which means looping over the basic blocks. Each basic block is a list of statements which imply expressions — that's [what we compile next](#compiling-statements-and-expressions). After each block, we also need to compile its [terminator](#compiling-terminators).

#### FunctionCtx

The `FunctionCtx` is a helper struct (which can be found [here](../../src/codegen/function_ctx.rs#25)). It is used to keep track of the current function's scope. When it's created, it initializes all the blocks that will be needed to compile a function whose size is taken from MATHIR. It also creates the operations necessary to allocate enough space on the stack to hold the function's parameters.

#### Locals

Locals refer, obviously, to variables and for that reason we need to make sure we keep track of their values. To do this, `FunctionCtx` has a `locals` hash map that maps the index of the local (based on MATHIR) to a pointer on the stack where the actual value was stored. To achieve this, we use `llvm.alloca` and `llvm.store` operations from the LLVM dialect to allocate the pointer and store the value in that pointer. Whenever we want to reference the value, we would use the `llvm.load` operation.

### Compiling Statements and Expressions

Statements handle variable declarations and assignments. A declaration allocates stack space with `llvm.alloca` and stores the initial value with `llvm.store`. An assignment loads the existing value, applies any operation, and stores the result back. Symbolic declarations create `symbolic.sym` operations. The entrypoint for statements is [lvalue.rs](../../src/codegen/lvalue.rs#23).

Expressions produce values. Constants become `arith.constant`, variable references become `llvm.load`, arithmetic operations become `arith.addi`/`arith.subi`/etc., and symbolic arithmetic becomes `symbolic.add`/`symbolic.sub`. The entrypoint for expressions is [rvalue.rs](../../src/codegen/rvalue.rs#27).

Control flow (if, for, while) is not compiled to instructions — it simply creates blocks with the appropriate terminators, matching the same structure that MATHIR already has.

### Compiling Terminators

MLIR enforces that every block ends with a terminator (an instruction that moves the control flow between blocks). MATHIR uses classic terminators like branch and conditional branch. For this, the [Control Flow Dialect](https://mlir.llvm.org/docs/Dialects/ControlFlowDialect/) is used, which provides `cf.br` and `cf.cond_br` respectively.

#### Function Call special Case

Function calls are not terminators in MLIR, however, Mathic represents them as a terminators for simplicity. When the function call is performed, the result is stored as a local and, since it's not actually a terminator, a `cf.br` operation is added to branch to the next block. You can check its implementation [here](../../src/codegen/terminator.rs#85).

### Passes

Once the MLIR module is generated, it needs to go through a pipeline of passes that lower it to LLVM IR. The pipeline is defined [here](../../src/compiler.rs#182) and consists of:

1. **Canonicalizer**: MLIR's built-in pass that simplifies the IR by folding constants and removing dead code.
2. **scf-to-cf**: converts structured control flow operations like `scf.for` and `scf.if` into unstructured branches (`cf.br`, `cf.cond_br`).
3. **symbolic-extract-eval** and **symbolic-to-arith**: Mathic-specific passes that lower the `symbolic` dialect to `arith` and `func`. See [Symbolic Passes](../dialects/SymbolicPasses.md) for a detailed explanation.
4. **convert-to-llvm**: lowers all remaining dialects to the LLVM dialect, which maps directly to LLVM IR.

After these passes the module contains only LLVM dialect operations, ready to be executed.

### JIT Engine

The lowered module is passed to the JIT ExecutionEngine, which compiles it to native code and runs it. Mathic wraps this in the `MathicExecutor` struct, defined [here](../../src/executor.rs).

The executor loads the module into memory and provides a `call_function` method that looks up a function by name, transmutes it to a native function pointer, and calls it. The result is returned as an `i64`. The entrypoint follows the naming convention `mathic__main`.