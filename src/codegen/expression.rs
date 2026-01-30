use melior::{
    helpers::ArithBlockExt,
    ir::{Block, Location, Value},
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    error::{MathicError, Result},
    parser::grammar::expression::{ExprStmt, PrimaryExpr},
};

impl MathicCodeGen {
    pub fn compile_expression<'ctx: 'this, 'this>(
        &self,
        block: &'this Block<'ctx>,
        expr: ExprStmt,
    ) -> Result<Value<'ctx, 'this>> {
        match expr {
            ExprStmt::Primary(primary_expr) => self.compile_primary(block, primary_expr),
            ExprStmt::Assign { name, value } => {
                Err(MathicError::Codegen(CodegenError::MeliorError(
                    melior::Error::Operation("Assignment not implemented".into()),
                )))
            }
            ExprStmt::BinOp { lhs, op, rhs } => {
                Err(MathicError::Codegen(CodegenError::MeliorError(
                    melior::Error::Operation("Binary operation not implemented".into()),
                )))
            }
            ExprStmt::Logical { lhs, op, rhs } => {
                Err(MathicError::Codegen(CodegenError::MeliorError(
                    melior::Error::Operation("Logical operation not implemented".into()),
                )))
            }
            ExprStmt::Unary { op, rhs } => Err(MathicError::Codegen(CodegenError::MeliorError(
                melior::Error::Operation("Unary operation not implemented".into()),
            ))),
            ExprStmt::Call { calle, args } => Err(MathicError::Codegen(CodegenError::MeliorError(
                melior::Error::Operation("Function call not implemented".into()),
            ))),
            ExprStmt::Index { name, pos } => Err(MathicError::Codegen(CodegenError::MeliorError(
                melior::Error::Operation("Indexing not implemented".into()),
            ))),
        }
    }

    pub fn compile_primary<'ctx: 'this, 'this>(
        &self,
        block: &'this Block<'ctx>,
        expr: PrimaryExpr,
    ) -> Result<Value<'ctx, 'this>> {
        let location = Location::unknown(&self.context);

        match expr {
            PrimaryExpr::Ident(_token) => Err(MathicError::Codegen(CodegenError::MeliorError(
                melior::Error::Operation("Identifier lookup not implemented".into()),
            ))),
            PrimaryExpr::Num(val) => {
                let parsed_val: u64 = val.parse()?;
                Ok(block.const_int(&self.context, location, parsed_val, 64))
            }
            PrimaryExpr::Str(_) => Err(MathicError::Codegen(CodegenError::MeliorError(
                melior::Error::Operation("String literals not implemented".into()),
            ))),
            PrimaryExpr::Bool(val) => Ok(block.const_int(&self.context, location, val.into(), 1)),
        }
    }
}
