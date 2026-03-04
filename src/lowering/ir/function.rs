use std::collections::HashMap;

use super::basic_block::{BasicBlock, BlockId, write_block_ir};
use crate::{
    diagnostics::LoweringError,
    lowering::ir::{
        DeclTable, IrBuilder, basic_block::Terminator, instruction::LValInstruct, types::MathicType,
    },
    parser::{Span, ast::declaration::Param},
};

/// A function in the IR
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub sym_table: SymbolTable,
    pub basic_blocks: Vec<BasicBlock>,
    pub params_tys: Vec<MathicType>,
    pub return_ty: MathicType,
    pub span: Span,
}

pub struct FunctionBuilder<'ir> {
    pub name: String,
    pub decl_table: DeclTable,
    pub sym_table: SymbolTable,
    pub params_tys: Vec<MathicType>,
    pub basic_blocks: Vec<BasicBlock>,
    pub return_ty: MathicType,
    pub ir_builder: &'ir mut IrBuilder,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalKind {
    Param,
    Temp,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Local {
    pub local_idx: usize,
    pub kind: LocalKind,
    pub ty: MathicType,
    pub debug_name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub locals: Vec<Local>,
    pub local_indexes: HashMap<String, usize>,
    pub functions: HashMap<String, Function>,
}

impl<'ir> FunctionBuilder<'ir> {
    /// Create a new function
    pub fn new(
        name: String,
        params: &[Param],
        return_ty: MathicType,
        ir_builder: &'ir mut IrBuilder,
        span: Span,
    ) -> Self {
        let mut func = Self {
            name,
            decl_table: DeclTable::default(),
            sym_table: Default::default(),
            basic_blocks: vec![BasicBlock::new(0, Terminator::Return(None, None), None)],
            params_tys: Vec::with_capacity(params.len()),
            return_ty,
            ir_builder,
            span,
        };

        for (param_idx, param) in params.iter().enumerate() {
            func.params_tys.push(param.ty);

            func.sym_table.locals.push(Local {
                local_idx: param_idx,
                kind: LocalKind::Param,
                ty: param.ty,
                debug_name: Some(param.name.clone()),
            });

            func.sym_table
                .local_indexes
                .insert(param.name.clone(), param_idx);
        }

        func
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

    /// Add a function declaration to the declaration table
    pub fn add_func_decl(&mut self, func: crate::parser::ast::declaration::FuncDecl) {
        self.decl_table.functions.insert(func.name.clone(), func);
    }
    /// Adds a user-defined local.
    pub fn add_local(
        &mut self,
        debug_name: Option<String>,
        ty: MathicType,
        span: Option<Span>,
        kind: LocalKind,
    ) -> Result<usize, LoweringError> {
        if let Some(name) = &debug_name
            && self.sym_table.local_indexes.contains_key(name)
        {
            return Err(LoweringError::DuplicateDeclaration {
                name: name.clone(),
                span: span.unwrap(),
            });
        }

        let idx = self.sym_table.locals.len();

        self.sym_table.locals.push(Local {
            local_idx: idx,
            kind,
            ty,
            debug_name: debug_name.clone(),
        });

        if let Some(name) = debug_name {
            self.sym_table.local_indexes.insert(name, idx);
        }

        Ok(idx)
    }

    pub fn add_function(&mut self, func: Function) {
        self.sym_table.functions.insert(func.name.clone(), func);
    }

    pub fn get_local_from_name(&self, name: &str, span: Span) -> Result<Local, LoweringError> {
        let local_idx = self.sym_table.local_indexes.get(name).copied().ok_or(
            LoweringError::UndeclaredVariable {
                name: name.to_string(),
                span,
            },
        )?;

        Ok(self.sym_table.locals[local_idx].clone())
    }

    #[allow(dead_code)]
    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.sym_table.functions.get(name)
    }

    /// Add a basic block
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

pub fn write_function_ir<W: std::fmt::Write>(
    func: &Function,
    f: &mut W,
    indent: usize,
) -> std::fmt::Result {
    let indent_str = " ".repeat(indent);

    let params = func
        .sym_table
        .locals
        .iter()
        .filter(|local| matches!(local.kind, LocalKind::Param))
        .map(|p| format!("%{}", p.local_idx))
        .collect::<Vec<_>>()
        .join(", ");

    writeln!(f, "{}df {}({}) -> i64 {{", indent_str, func.name, params)?;

    for (_, nested_func) in func.sym_table.functions.iter() {
        write_function_ir(nested_func, f, indent + 4)?;
    }

    for block in func.basic_blocks.iter() {
        write_block_ir(block, f, indent + 4)?;
    }

    writeln!(f, "{}}}\n", indent_str)
}
