use std::{io::Write, path::PathBuf};

use melior::{
    Context,
    dialect::DialectRegistry,
    ir::{
        Attribute, AttributeLike, Block, Identifier, Location, Module, Region, RegionLike,
        attribute::StringAttribute,
        operation::{OperationBuilder, OperationLike},
    },
    pass::{
        PassManager,
        conversion::{create_scf_to_control_flow, create_to_llvm},
        transform::create_canonicalizer,
    },
    utility::{register_all_dialects, register_all_llvm_translations, register_all_passes},
};
use mlir_sys::{
    MlirLLVMDIEmissionKind_MlirLLVMDIEmissionKindFull,
    MlirLLVMDINameTableKind_MlirLLVMDINameTableKindDefault, mlirDisctinctAttrCreate,
    mlirLLVMDICompileUnitAttrGet, mlirLLVMDIFileAttrGet, mlirLLVMDIModuleAttrGet,
};
use std::sync::OnceLock;

use llvm_sys::target::{
    LLVM_InitializeAllAsmPrinters, LLVM_InitializeAllTargetInfos, LLVM_InitializeAllTargetMCs,
    LLVM_InitializeAllTargets,
};
use std::{fs, path::Path};

use crate::{
    MathicResult,
    codegen::{MathicCodeGen, error::CodegenError},
    ffi,
    parser::MathicParser,
};

#[derive(Default)]
#[repr(u8)]
pub enum OptLvl {
    None,
    #[default]
    O1,
    O2,
    O3,
}

pub struct MathicCompiler {
    ctx: Context,
}

impl MathicCompiler {
    pub fn new() -> Result<Self, CodegenError> {
        let ctx = Self::create_context()?;

        Ok(Self { ctx })
    }

    pub fn compile_path(&self, file_path: &Path, opt_lvl: OptLvl) -> MathicResult<Module<'_>> {
        // Read source file
        let source = fs::read_to_string(file_path)?;
        self.compile_source(&source, opt_lvl, Some(file_path))
    }

    pub fn compile_source(
        &self,
        source: &str,
        _opt_lvl: OptLvl,
        file_path: Option<&Path>,
    ) -> MathicResult<Module<'_>> {
        // Parse the source code
        let parser = MathicParser::new(source);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(e) => {
                if let Some(path) = file_path {
                    parser.format_error(path, &e);
                }
                return Err(e.into());
            }
        };
        dbg!(&ast);
        // Generate MLIR code
        let mut module = Self::create_module(&self.ctx)?;
        let mut codegen = MathicCodeGen::new(&module);

        // Generate the main module.
        codegen.generate_module(&self.ctx, ast)?;

        if let Ok(v) = std::env::var("MATHIC_DBG_DUMP") {
            if v == "1" {
                let file_path = PathBuf::from("dump-prepass.mlir");
                let mut f = fs::File::create(file_path).unwrap();
                write!(f, "{}", module.as_operation()).unwrap();
            } else {
                tracing::warn!(
                    "Incorrect value for MATHIC_DBG_DUMP: \"{}\", igonring it",
                    v
                )
            }
        }

        tracing::debug!("Module Done");
        debug_assert!(module.as_operation().verify());

        // Run Passes to the generated module.
        self.run_passes(&mut module)?;

        tracing::debug!("Passes Done");

        if let Ok(v) = std::env::var("MATHIC_DBG_DUMP") {
            if v == "1" {
                let file_path = PathBuf::from("dump.mlir");
                let mut f = fs::File::create(file_path).unwrap();
                write!(f, "{}", module.as_operation()).unwrap();
            } else {
                tracing::warn!(
                    "Incorrect value for MATHIC_DBG_DUMP: \"{}\", igonring it",
                    v
                )
            }
        }

        Ok(module)
    }

    fn create_module(ctx: &Context) -> Result<Module<'_>, CodegenError> {
        static INITIALIZED: OnceLock<()> = OnceLock::new();

        INITIALIZED.get_or_init(|| unsafe {
            LLVM_InitializeAllTargets();
            LLVM_InitializeAllTargetInfos();
            LLVM_InitializeAllTargetMCs();
            LLVM_InitializeAllAsmPrinters();
        });

        let target_triple = ffi::get_target_triple();

        let module_region = Region::new();
        module_region.append_block(Block::new(&[]));

        let data_layout_ret = &ffi::get_data_layout_rep()?;

        let di_unit_id = unsafe {
            let id = StringAttribute::new(ctx, "compile_unit_id").to_raw();
            mlirDisctinctAttrCreate(id)
        };

        let op = OperationBuilder::new(
            "builtin.module",
            Location::fused(ctx, &[Location::new(ctx, "program.mth", 0, 0)], {
                let file_attr = unsafe {
                    Attribute::from_raw(mlirLLVMDIFileAttrGet(
                        ctx.to_raw(),
                        StringAttribute::new(ctx, "program.mth").to_raw(),
                        StringAttribute::new(ctx, "").to_raw(),
                    ))
                };
                unsafe {
                    let di_unit = mlirLLVMDICompileUnitAttrGet(
                        ctx.to_raw(),
                        di_unit_id,
                        0x1c, // rust
                        file_attr.to_raw(),
                        StringAttribute::new(ctx, "mathic").to_raw(),
                        false,
                        MlirLLVMDIEmissionKind_MlirLLVMDIEmissionKindFull,
                        MlirLLVMDINameTableKind_MlirLLVMDINameTableKindDefault,
                    );

                    let di_module = mlirLLVMDIModuleAttrGet(
                        ctx.to_raw(),
                        file_attr.to_raw(),
                        di_unit,
                        StringAttribute::new(ctx, "LLVMDialectModule").to_raw(),
                        StringAttribute::new(ctx, "").to_raw(),
                        StringAttribute::new(ctx, "").to_raw(),
                        StringAttribute::new(ctx, "").to_raw(),
                        0,
                        false,
                    );

                    Attribute::from_raw(di_module)
                }
            }),
        )
        .add_attributes(&[
            (
                Identifier::new(ctx, "llvm.target_triple"),
                StringAttribute::new(ctx, &target_triple).into(),
            ),
            (
                Identifier::new(ctx, "llvm.data_layout"),
                StringAttribute::new(ctx, data_layout_ret).into(),
            ),
        ])
        .add_regions([module_region])
        .build()?;

        Module::from_operation(op)
            .ok_or(CodegenError::Custom("Could not create module".to_string()))
    }

    fn create_context() -> Result<Context, CodegenError> {
        let ctx = Context::new();

        ctx.append_dialect_registry(&Self::create_dialect_registry());
        ctx.load_all_available_dialects();

        register_all_passes();
        register_all_llvm_translations(&ctx);

        Ok(ctx)
    }

    fn run_passes(&self, module: &mut Module) -> Result<(), CodegenError> {
        let pass_manager = PassManager::new(&self.ctx);

        pass_manager.enable_verifier(true);
        pass_manager.add_pass(create_canonicalizer());
        pass_manager.add_pass(create_scf_to_control_flow()); // needed because to_llvm doesn't include it.
        pass_manager.add_pass(create_to_llvm());

        pass_manager.run(module)?;

        Ok(())
    }

    fn create_dialect_registry() -> DialectRegistry {
        let registry = DialectRegistry::new();

        register_all_dialects(&registry);

        registry
    }
}
