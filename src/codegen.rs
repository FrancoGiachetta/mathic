use std::{fs, path::PathBuf};

use ariadne::Source;
use melior::{
    Context,
    dialect::llvm,
    ir::{Location, Module, Type, r#type::IntegerType},
};

use crate::{
    MathicResult,
    codegen::{compiler_helper::CompilerHelper, dialect_integration::symbolic},
    diagnostics::{CodegenError, MathicError},
    lowering::ir::{
        Ir,
        function::Function,
        symbols::TypeIndex,
        types::{FloatTy, MathicType, NumericTy},
    },
    parser::Span,
};
use tracing::instrument;

pub mod compiler_helper;
mod dialect_integration;
pub mod function_ctx;
pub mod lvalue;
pub mod rvalue;
pub mod terminator;

/// Struct that holds global infomation to the code generation.
///
/// ## Fields
///
/// **ctx**: MLIR Context, global to the whole compilation.
/// **module**: MLIR Module, where we store the generated mlir code.
/// **file_path**: the path to file being compiled.
pub struct MathicCodeGen<'ctx> {
    ctx: &'ctx Context,
    module: &'ctx Module<'ctx>,
    ir: &'ctx Ir,
    file_path: Option<PathBuf>,
}

impl<'ctx> MathicCodeGen<'ctx> {
    pub fn new(
        ctx: &'ctx Context,
        ir: &'ctx Ir,
        module: &'ctx Module<'ctx>,
        file_path: Option<PathBuf>,
    ) -> Self {
        Self {
            ctx,
            module,
            ir,
            file_path,
        }
    }

    /// Returns a melior location.
    ///
    /// The location is relative to the file being compiled.
    pub fn get_location(&self, span: Option<Span>) -> Result<Location<'ctx>, CodegenError> {
        Ok(
            if let (Some(path), Some(span)) = (self.file_path.as_ref(), span) {
                let (_, line, column) = {
                    let source = fs::read_to_string(path)?;
                    Source::from(source).get_offset_line(span.start).unwrap()
                };
                Location::new(
                    self.ctx,
                    path.file_name().unwrap().to_str().unwrap(),
                    line,
                    column,
                )
            } else {
                Location::unknown(self.ctx)
            },
        )
    }

    /// Code generation entrypoint.
    ///
    /// Populates the module for a compile unit.
    #[instrument(target = "codegen", skip(self, helper))]
    pub fn generate_module(&self, helper: &mut CompilerHelper) -> MathicResult<()> {
        let start = std::time::Instant::now();
        let global_functions = self.ir.get_functions();

        tracing::info!(
            "Starting code generation for {} functions",
            global_functions.len()
        );

        // Check if main function is present
        if !global_functions.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        for func in global_functions {
            tracing::debug!("Compiling function: {}", func.name);
            self.compile_function(func, &[], helper)?;
        }

        tracing::info!("Code generation complete: {:?}", start.elapsed());

        Ok(())
    }

    pub fn get_type(&self, func: &Function, ty_idx: TypeIndex) -> Result<MathicType, CodegenError> {
        let ty = if ty_idx.is_local {
            func.get_type(ty_idx.idx)
        } else {
            self.ir.get_type(ty_idx.idx)
        };
        ty.ok_or(CodegenError::InvalidTypeIndex(ty_idx.idx))
    }

    pub fn get_compiled_type<'func>(
        &'func self,
        func: &Function,
        ty_idx: TypeIndex,
    ) -> Result<Type<'func>, CodegenError> {
        let ty = self.get_type(func, ty_idx)?;

        Ok(match ty {
            MathicType::Numeric(NumericTy::Sint(_) | NumericTy::Uint(_)) => {
                IntegerType::new(self.ctx, ty.bit_width()).into()
            }
            MathicType::Numeric(NumericTy::Float(float_ty)) => match float_ty {
                FloatTy::F32 => Type::float32(self.ctx),
                FloatTy::F64 => Type::float64(self.ctx),
            },
            MathicType::Bool => IntegerType::new(self.ctx, 1).into(),
            MathicType::Char => IntegerType::new(self.ctx, 8).into(),
            MathicType::Str => llvm::r#type::pointer(self.ctx, 0),
            MathicType::SymbolicExpr(inner_ty) => {
                let inner_type = match inner_ty {
                    NumericTy::Sint(_) | NumericTy::Uint(_) => {
                        IntegerType::new(self.ctx, inner_ty.bit_width()).into()
                    }
                    NumericTy::Float(FloatTy::F32) => Type::float32(self.ctx),
                    NumericTy::Float(FloatTy::F64) => Type::float64(self.ctx),
                };
                symbolic::sym_expr_type(self.ctx, inner_type, inner_ty.is_signed())
            }
            MathicType::Void => Type::none(self.ctx),
            MathicType::Adt { index, is_local } => {
                let adt = if is_local {
                    func.get_adt(index)
                } else {
                    self.ir.get_adt(index)
                }
                .ok_or(CodegenError::InvalidAdtIndex(index))?;

                let fields_tys = adt
                    .get_fields_tys()
                    .iter()
                    .map(|ty| self.get_compiled_type(func, *ty))
                    .collect::<Result<Vec<_>, _>>()?;

                llvm::r#type::r#struct(self.ctx, &fields_tys, false)
            }
        })
    }
}
