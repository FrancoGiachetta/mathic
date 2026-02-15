use melior::{
    dialect::func,
    ir::{
        Block, BlockLike, Location, Region, RegionLike,
        attribute::{StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    parser::ast::declaration::{DeclStmt, FuncDecl},
};

impl MathicCodeGen<'_> {
    pub fn compile_function(&self, func: FuncDecl) -> Result<(), CodegenError> {
        // let params = vec![];

        let region = Region::new();
        let inner_block = region.append_block(Block::new(&[]));

        for stmt in func.body.iter() {
            self.compile_statement(&block, stmt)?;

            dbg!("STMT");
        }

        let location = Location::unknown(self.ctx);
        let i64_type = IntegerType::new(self.ctx, 64).into();

        self.module.body().append_operation(func::func(
            self.ctx,
            StringAttribute::new(self.ctx, &format!("mathic__{}", func.name)),
            TypeAttribute::new(FunctionType::new(self.ctx, &[], &[i64_type]).into()),
            region,
            // This is necessary for the ExecutorEngine to execute a function.
            &[(
                Identifier::new(self.ctx, "llvm.emit_c_interface"),
                Attribute::unit(self.ctx),
            )],
            location,
        ));

        Ok(())
    }
}
