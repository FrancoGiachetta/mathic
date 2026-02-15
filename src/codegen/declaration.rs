use melior::{
    dialect::func,
    ir::{
        Block, BlockLike, Location, Region, RegionLike,
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
        let value = self.compile_expression(block, expr)?;

        self.define_sym(name.to_string(), value);

        Ok(())
    }

    pub fn compile_function(&self, block: &Block, func: &FuncDecl) -> Result<(), CodegenError> {
        // let params = vec![];

        let region = Region::new();
        let inner_block = region.append_block(Block::new(&[]));

        for stmt in func.body.iter() {
            self.compile_statement(&inner_block, stmt)?;

            dbg!("STMT");
        }

        let location = Location::unknown(self.ctx);
        let i64_type = IntegerType::new(self.ctx, 64).into();

        block.append_operation(func::func(
            self.ctx,
            StringAttribute::new(self.ctx, &format!("mathic__{}", func.name)),
            TypeAttribute::new(FunctionType::new(self.ctx, &[], &[i64_type]).into()),
            region,
            &[],
            location,
        ));

        Ok(())
    }
}
