use super::basic_block::{BasicBlock, BlockId};
use crate::{
    diagnostics::LoweringError,
    lowering::ir::{
        DeclTable, IrBuilder,
        adts::Adt,
        basic_block::Terminator,
        instruction::LValInstruct,
        symbols::{LocalSymbolTable, TypeIndex},
        types::{MathicType, lower_inner_ast_type},
    },
    parser::{
        Span,
        ast::declaration::{FuncDecl, Param},
    },
};

/// MATHIR's representation of a function.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub sym_table: SymbolTable,
    pub basic_blocks: Vec<BasicBlock>,
    pub params_tys: Vec<TypeIndex>,
    pub return_ty: TypeIndex,
    pub span: Span,
}

/// Helper struct to build a Function.
pub struct FunctionBuilder<'glb> {
    pub name: String,
    pub decl_table: DeclTable,
    pub sym_table: LocalSymbolTable<'glb>,
    pub params_tys: Vec<TypeIndex>,
    pub basic_blocks: Vec<BasicBlock>,
    pub return_ty: TypeIndex,
    pub ir_builder: &'glb mut IrBuilder,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalKind {
    Param,
    Temp,
}

/// MATHIR's representation of local variables.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Local {
    pub local_idx: usize,
    pub kind: LocalKind,
    pub ty: TypeIndex,
    pub debug_name: Option<String>,
}

impl<'ir> FunctionBuilder<'ir> {
    /// Create a new function
    pub fn new(
        name: String,
        params: &[Param],
        return_ty: TypeIndex,
        ir_builder: &'ir mut IrBuilder,
        span: Span,
    ) -> Result<Self, LoweringError> {
        let mut func_builder = Self {
            name,
            decl_table: DeclTable::default(),
            sym_table: LocalSymbolTable::new(&mut ir_builder.sym_table),
            basic_blocks: vec![BasicBlock::new(0, Terminator::Return(None, None), None)],
            params_tys: Vec::new(),
            return_ty,
            ir_builder,
            span,
        };

        for param in params.iter() {
            let param_ty = lower_inner_ast_type(&mut func_builder, &param.ty, param.span)?;

            func_builder.params_tys.push(param_ty);

            let param_idx = func_builder.sym_table.add_local(
                Some(param.name.clone()),
                param_ty,
                Some(span),
                LocalKind::Param,
            )?;
            func_builder
                .sym_table
                .local_indexes
                .insert(param.name.clone(), param_idx);
        }

        Ok(func_builder)
    }

    /// Build the function and add it to the IR builder
    pub fn build(self) -> Function {
        Function {
            name: self.name,
            sym_table: self.sym_table,
            params_tys: self.params_tys,
            basic_blocks: self.basic_blocks,
            return_ty: self.return_ty,
            span: self.span,
        }
    }

    pub fn get_function_decl(&self, name: &str, span: Span) -> Result<FuncDecl, LoweringError> {
        match self.decl_table.get_function_decl(name).cloned() {
            Some(f) => Ok(f),
            None => match self.ir_builder.decl_table.get_function_decl(name).cloned() {
                Some(f) => Ok(f),
                None => Err(LoweringError::UndeclaredFunction {
                    name: name.to_string(),
                    span,
                }),
            },
        }
    }

    pub fn get_user_def_type(&self, name: &str, span: Span) -> Result<TypeIndex, LoweringError> {
        if let Some(ty) = self.sym_table.get_user_def_type(name) {
            return Ok(ty);
        }
        if let Some(ty) = self.ir_builder.get_user_def_type(name) {
            return Ok(ty);
        }

        Err(LoweringError::UndeclaredType { span })
    }

    pub fn get_adt_body(&self, adt_ty: MathicType, span: Span) -> Result<&Adt, LoweringError> {
        let MathicType::Adt { index, is_local } = adt_ty else {
            panic!()
        };

        let adt = if is_local {
            self.sym_table.get_adt(index)
        } else {
            self.ir_builder.adts.get(index)
        };

        adt.ok_or(LoweringError::UndeclaredType { span })
    }

    pub fn add_block(&mut self, terminator: Terminator, span: Option<Span>) -> BlockId {
        let id = self.basic_blocks.len();

        self.basic_blocks.push(BasicBlock {
            id,
            instructions: Vec::new(),
            terminator,
            span,
        });

        id
    }

    pub fn push_instruction(&mut self, inst: LValInstruct) {
        let last_index = self.basic_blocks.len() - 1;
        self.basic_blocks[last_index].instructions.push(inst);
    }

    pub fn get_basic_block_mut(&mut self, idx: usize) -> &mut BasicBlock {
        &mut self.basic_blocks[idx]
    }

    pub fn last_block_idx(&self) -> BlockId {
        self.basic_blocks.len() - 1
    }
}
