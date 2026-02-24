use melior::{
    helpers::LlvmBlockExt,
    ir::{Block, Location, r#type::IntegerType},
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError, function_ctx::FunctionCtx},
    lowering::ir::instruction::LValInstruct,
};

impl MathicCodeGen<'_> {
    pub fn compile_statement<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        inst: &LValInstruct,
    ) -> Result<(), CodegenError>
    where
        'func: 'ctx,
    {
        let location = Location::unknown(self.ctx);

        match inst {
            LValInstruct::Let {
                local_idx: _, init, ..
            } => {
                let init_val = self.compile_rvalue(fn_ctx, block, init)?;
                let ptr =
                    block.alloca1(self.ctx, location, IntegerType::new(self.ctx, 64).into(), 8)?;

                block.store(self.ctx, location, ptr, init_val)?;

                fn_ctx.define_local(ptr);
            }
            LValInstruct::Assign {
                local_idx, value, ..
            } => {
                let val = self.compile_rvalue(fn_ctx, block, value)?;
                let ptr = fn_ctx.get_local(*local_idx).expect("invalid local idx");

                block.store(self.ctx, location, ptr, val)?;
            }
        }

        Ok(())
    }

    pub fn compile_block<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        stmts: &[LValInstruct],
    ) -> Result<(), CodegenError>
    where
        'func: 'ctx,
    {
        for stmt in stmts {
            self.compile_statement(fn_ctx, block, stmt)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::compile_and_execute;
    use rstest::*;

    #[rstest]
    #[case("df main() { return 0; }", 0)]
    #[case("df main() { return 42; }", 42)]
    #[case("df main() { return true; }", 1)]
    #[case("df main() { return false; }", 0)]
    fn test_return_statements(#[case] source: &str, #[case] expected: i64) {
        assert_eq!(compile_and_execute(source), expected);
    }

    #[rstest]
    #[case("df main() { return 42 == 42; }", 1)]
    #[case("df main() { return 42 != 21; }", 1)]
    #[case("df main() { return true and false; }", 0)]
    #[case("df main() { return true or false; }", 1)]
    fn test_return_with_expressions(#[case] source: &str, #[case] expected: i64) {
        assert_eq!(compile_and_execute(source), expected);
    }
}
