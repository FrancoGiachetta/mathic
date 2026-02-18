use melior::{
    dialect::func,
    helpers::{BuiltinBlockExt, LlvmBlockExt},
    ir::{
        Block, BlockLike, Location, Region, RegionLike, ValueLike,
        attribute::{StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError, symbol_table::SymbolTable},
    parser::ast::declaration::{DeclStmt, FuncDecl, VarDecl},
};

impl MathicCodeGen<'_> {
    pub fn compile_declaration(&self, block: &Block, stmt: &DeclStmt) -> Result<(), CodegenError> {
        match stmt {
            DeclStmt::Var(var_decl) => self.compile_var_decl(block, var_decl),
            DeclStmt::Struct(_struct_decl) => todo!(),
            DeclStmt::Func(func_decl) => {
                let old_sym_table = self
                    .sym_table
                    .replace(SymbolTable::with_parent(self.sym_table.clone()));
                self.compile_function(block, func_decl)?;
                self.sym_table.replace(old_sym_table);

                Ok(())
            }
        }
    }

    pub fn compile_var_decl(
        &self,
        block: &Block,
        VarDecl { name, expr }: &VarDecl,
    ) -> Result<(), CodegenError> {
        let location = Location::unknown(self.ctx);

        let value = self.compile_expression(block, expr)?;

        let ptr = block.alloca1(self.ctx, location, value.r#type(), 8)?;
        block.store(self.ctx, location, ptr, value)?;

        self.define_sym(name.to_string(), ptr);

        Ok(())
    }

    pub fn compile_function(&self, block: &Block, func: &FuncDecl) -> Result<(), CodegenError> {
        let location = Location::unknown(self.ctx);
        let i64_type = IntegerType::new(self.ctx, 64).into();

        let mut params_types = Vec::with_capacity(func.params.len());
        let mut block_params = Vec::with_capacity(func.params.len());

        for _ in func.params.iter() {
            params_types.push(IntegerType::new(self.ctx, 64).into());
            block_params.push((IntegerType::new(self.ctx, 64).into(), location));
        }

        let region = Region::new();
        let inner_block = region.append_block(Block::new(&block_params));

        // Allocate space for parameters and make them visible to the function
        for (i, param) in func.params.iter().enumerate() {
            let value = inner_block.arg(i)?;
            let ptr = inner_block.alloca1(self.ctx, location, params_types[i], 8)?;

            inner_block.store(self.ctx, location, ptr, value)?;

            self.define_sym(param.name.to_string(), ptr);
        }

        for stmt in func.body.iter() {
            self.compile_statement(&inner_block, stmt)?;
        }

        block.append_operation(func::func(
            self.ctx,
            StringAttribute::new(self.ctx, &format!("mathic__{}", func.name)),
            TypeAttribute::new(FunctionType::new(self.ctx, &params_types, &[i64_type]).into()),
            region,
            &[],
            location,
        ));

        Ok(())
    }
}
