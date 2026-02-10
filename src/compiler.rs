use std::io::Write;

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

pub struct MathicCompiler<'a> {
    ctx: Context,
    pass_manager: PassManager<'a>,
}

impl<'a> MathicCompiler<'a> {
    pub fn new() -> Result<Self, CodegenError> {
        let ctx = Self::create_context()?;

        let pass_manager = PassManager::new(&ctx);

        pass_manager.enable_verifier(true);
        pass_manager.add_pass(create_canonicalizer());
        pass_manager.add_pass(create_scf_to_control_flow()); // needed because to_llvm doesn't include it.
        pass_manager.add_pass(create_to_llvm());

        Ok(Self { ctx, pass_manager })
    }

    pub fn compile(&'a self, file_path: &'a Path, _opt_lvl: OptLvl) -> MathicResult<Module<'a>> {
        // Read source file
        let source = fs::read_to_string(file_path)?;

        // Parse the source code
        let parser = MathicParser::new(&source);
        let ast = parser.parse()?;

        // Generate MLIR code
        let mut module = Self::create_module(&self.ctx)?;
        let mut codegen = MathicCodeGen::new(&self.ctx, &module);

        // Generate the main module.
        codegen.generate_module(ast)?;

        tracing::debug!("Module Done");
        debug_assert!(module.as_operation().verify());

        // Run Passes to the generated module.
        self.run_passes(&mut module)?;

        tracing::debug!("Passes Done");

        if let Ok(v) = std::env::var("MATHIC_DBG_DUMP") {
            if v == "1" {
                let mut f = fs::File::create(file_path.with_extension("mlir")).unwrap();
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

    fn create_module(ctx: &'a Context) -> Result<Module<'a>, CodegenError> {
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
        self.pass_manager.run(module)?;

        Ok(())
    }

    fn create_dialect_registry() -> DialectRegistry {
        let registry = DialectRegistry::new();

        register_all_dialects(&registry);

        registry
    }
}
