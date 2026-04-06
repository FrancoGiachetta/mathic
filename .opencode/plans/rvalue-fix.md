# Fix: rvalue.rs compile_value_use - ptr not updated in modifier loop

## Problem
`compile_value_use` in `src/codegen/rvalue.rs:242-313` has two bugs:

1. **`ptr` never updated**: In the `ValueModifier::Index` case, `gep` computes a new pointer (`elem_ptr`) but `ptr` is never reassigned. Subsequent modifiers still use the original base pointer, causing `array[1][0]` to return `array[0][0]`.

2. **`Field` uses `extract_value`**: Inconsistent with `lvalue.rs` which uses `gep` for both `Field` and `Index`.

## Fix
Refactor to match the pattern in `lvalue.rs:58-103`:

1. Make `ptr` mutable: `let (mut ptr, mut ty_idx) = ...`
2. Remove the initial `block.load` before the loop
3. Replace `Field`'s `extract_value` with `gep` using `GepIndex::Const(*idx as i32)`
4. Replace `Index`'s `elem_ptr` with direct `ptr` assignment
5. Add single `block.load(...)` after the loop to get final value

## Changes in `src/codegen/rvalue.rs`

Replace lines 242-313 with:

```rust
IRValue::InMemory {
    local_idx,
    modifier,
} => {
    let (mut ptr, mut ty_idx) = fn_ctx.get_local(*local_idx).expect("Invalid local idx");

    for m in modifier {
        let ty = self.get_type(fn_ctx.get_ir_func(), ty_idx)?;

        ptr = match m {
            ValueModifier::Field(idx) => match ty {
                MathicType::Adt { index, is_local } => {
                    let adt = if is_local {
                        fn_ctx.get_ir_func().get_adt(index)
                    } else {
                        self.ir.get_adt(index)
                    }
                    .ok_or(CodegenError::InvalidAdtIndex(index))?;

                    match adt {
                        Adt::Struct(struct_adt) => {
                            let field_ty_idx = struct_adt.fields[*idx].ty;
                            ty_idx = field_ty_idx;
                            block.gep(
                                self.ctx,
                                location,
                                ptr,
                                &[GepIndex::Const(*idx as i32)],
                                self.get_compiled_type(fn_ctx.get_ir_func(), ty_idx)?,
                            )?
                        }
                    }
                }
                other => unreachable!("{}", other),
            },
            ValueModifier::Index(idx) => match ty {
                MathicType::Array { inner_ty_idx, .. } => {
                    ty_idx = inner_ty_idx;
                    let (index_ptr, index_ty) = fn_ctx.get_local(*idx).expect("");
                    let index_mlir_ty =
                        self.get_compiled_type(fn_ctx.get_ir_func(), index_ty)?;

                    block.gep(
                        self.ctx,
                        location,
                        ptr,
                        &[GepIndex::Value(block.load(
                            self.ctx,
                            location,
                            index_ptr,
                            index_mlir_ty,
                        )?)],
                        self.get_compiled_type(fn_ctx.get_ir_func(), ty_idx)?,
                    )?
                }
                other => unreachable!("{other}"),
            },
        };
    }

    block.load(
        self.ctx,
        location,
        ptr,
        self.get_compiled_type(fn_ctx.get_ir_func(), ty_idx)?,
    )?
```

## Verification
Run: `cargo run -- examples/arrays/array_init.mth`
Expected: `Ok(2)`
