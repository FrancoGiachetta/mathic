use std::cell::RefCell;

use melior::{
    Context,
    dialect::func,
    helpers::{BuiltinBlockExt, LlvmBlockExt},
    ir::{
        Block, BlockLike, Identifier, Location, Module, Region, RegionLike, Value,
        attribute::{Attribute, StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
};

use crate::{
    MathicResult,
    codegen::{error::CodegenError, symbol_table::SymbolTable},
    error::MathicError,
    parser::ast::{Program, declaration::FuncDecl},
};

pub mod control_flow;
pub mod declaration;
pub mod error;
pub mod expression;
pub mod statement;
pub mod symbol_table;

pub struct MathicCodeGen<'ctx> {
    ctx: &'ctx Context,
    module: &'ctx Module<'ctx>,
    sym_table: RefCell<SymbolTable>,
}

impl<'ctx> MathicCodeGen<'ctx> {
    pub fn new(ctx: &'ctx Context, module: &'ctx Module<'ctx>) -> Self {
        Self {
            ctx,
            module,
            sym_table: Default::default(),
        }
    }

    fn define_sym(&self, name: String, value: Value<'ctx, '_>) {
        self.sym_table.borrow_mut().insert(name, value);
    }

    fn get_sym(&self, name: &str) -> Result<Value<'ctx, '_>, CodegenError> {
        self.sym_table
            .borrow()
            .get(name)
            .map(|v| unsafe { Value::from_raw(v) })
            .ok_or(CodegenError::IdentifierNotFound(name.to_string()))
    }

    pub fn generate_module(&self, program: Program) -> MathicResult<()> {
        // Check if main function is present
        if !program.funcs.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        // TODO: Compile structs in the future

        for func in program.funcs {
            self.compile_entry_point(func)?;

            self.sym_table.replace(SymbolTable::new());
        }

        Ok(())
    }

    pub fn compile_entry_point(&self, func: FuncDecl) -> Result<(), CodegenError> {
        let location = Location::unknown(self.ctx);
        let i64_type = IntegerType::new(self.ctx, 64).into();

        let mut params_types = Vec::with_capacity(func.params.len());
        let mut block_params = Vec::with_capacity(func.params.len());

        for _ in func.params.iter() {
            params_types.push(IntegerType::new(self.ctx, 64).into());
            block_params.push((IntegerType::new(self.ctx, 64).into(), location));
        }

        let region = Region::new();
        let block = region.append_block(Block::new(&block_params));

        // Allocate space for parameters and make them visible to the function
        for (i, param) in func.params.iter().enumerate() {
            let value = block.arg(i)?;
            let ptr = block.alloca1(self.ctx, location, params_types[i], 8)?;

            block.store(self.ctx, location, ptr, value)?;

            self.define_sym(param.name.to_string(), ptr);
        }

        for stmt in func.body.iter() {
            self.compile_statement(&block, stmt)?;
        }

        self.module.body().append_operation(func::func(
            self.ctx,
            StringAttribute::new(self.ctx, &format!("mathic__{}", func.name)),
            TypeAttribute::new(FunctionType::new(self.ctx, &params_types, &[i64_type]).into()),
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
