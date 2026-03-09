use std::{
    ffi::CStr,
    mem::MaybeUninit,
    ptr::{addr_of_mut, null_mut},
    sync::OnceLock,
};

use llvm_sys::{
    core::LLVMDisposeMessage,
    target::{
        LLVM_InitializeAllAsmPrinters, LLVM_InitializeAllTargetInfos, LLVM_InitializeAllTargetMCs,
        LLVM_InitializeAllTargets, LLVMDisposeTargetData,
    },
    target_machine::{
        LLVMCodeGenOptLevel, LLVMCodeModel, LLVMCreateTargetMachine, LLVMDisposeTargetMachine,
        LLVMGetDefaultTargetTriple, LLVMGetHostCPUFeatures, LLVMGetHostCPUName,
        LLVMGetTargetFromTriple, LLVMRelocMode, LLVMTargetRef,
    },
};
use melior::{
    Context,
    dialect::DialectRegistry,
    ir::{
        Block, Identifier, Location, Module, Region, RegionLike, attribute::StringAttribute,
        operation::OperationBuilder,
    },
    utility::{register_all_dialects, register_all_llvm_translations, register_all_passes},
};

use crate::{compiler::OptLvl, diagnostics::CodegenError};

pub fn create_module<'ctx>(
    ctx: &'ctx Context,
    opt_lvl: OptLvl,
) -> Result<Module<'ctx>, CodegenError> {
    static INITIALIZED: OnceLock<()> = OnceLock::new();

    INITIALIZED.get_or_init(|| unsafe {
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmPrinters();
    });

    let target_triple = get_target_triple();

    let module_region = Region::new();
    module_region.append_block(Block::new(&[]));

    let data_layout_ret = &get_data_layout_rep(opt_lvl)?;

    let op = OperationBuilder::new("builtin.module", Location::unknown(ctx))
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

    Module::from_operation(op).ok_or(CodegenError::Custom("Could not create module".to_string()))
}

pub fn create_context() -> Result<Context, CodegenError> {
    let ctx = Context::new();

    ctx.append_dialect_registry(&create_dialect_registry());
    ctx.load_all_available_dialects();

    register_all_passes();
    register_all_llvm_translations(&ctx);

    Ok(ctx)
}

fn create_dialect_registry() -> DialectRegistry {
    let registry = DialectRegistry::new();

    register_all_dialects(&registry);

    registry
}

/// Gets the target triple, which identifies the platform and ABI.
pub fn get_target_triple() -> String {
    unsafe {
        let value = LLVMGetDefaultTargetTriple();
        let result = CStr::from_ptr(value).to_string_lossy().into_owned();
        LLVMDisposeMessage(value);
        result
    }
}

/// Gets the data layout reprrsentation as a string, to be given to the MLIR module.
/// LLVM uses this to know the proper alignments for the given sizes, etc.
/// This function gets the data layout of the host target triple.
pub fn get_data_layout_rep(opt_lvl: OptLvl) -> Result<String, CodegenError> {
    unsafe {
        let mut null = null_mut();
        let error_buffer = addr_of_mut!(null);

        let target_triple = LLVMGetDefaultTargetTriple();
        let target_cpu = LLVMGetHostCPUName();
        let target_cpu_features = LLVMGetHostCPUFeatures();

        let mut target: MaybeUninit<LLVMTargetRef> = MaybeUninit::uninit();

        if LLVMGetTargetFromTriple(target_triple, target.as_mut_ptr(), error_buffer) != 0 {
            let error = CStr::from_ptr(*error_buffer);
            let err = error.to_string_lossy().to_string();
            LLVMDisposeMessage(*error_buffer);
            LLVMDisposeMessage(target_triple);
            LLVMDisposeMessage(target_cpu);
            LLVMDisposeMessage(target_cpu_features);
            Err(CodegenError::LLVMError(err))?;
        }
        if !(*error_buffer).is_null() {
            LLVMDisposeMessage(*error_buffer);
        }

        let target = target.assume_init();

        let machine = LLVMCreateTargetMachine(
            target,
            target_triple.cast(),
            target_cpu.cast(),
            target_cpu_features.cast(),
            match opt_lvl {
                OptLvl::None => LLVMCodeGenOptLevel::LLVMCodeGenLevelNone,
                OptLvl::O1 => LLVMCodeGenOptLevel::LLVMCodeGenLevelLess,
                OptLvl::O2 => LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
                OptLvl::O3 => LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
            },
            LLVMRelocMode::LLVMRelocDefault,
            LLVMCodeModel::LLVMCodeModelDefault,
        );

        let data_layout = llvm_sys::target_machine::LLVMCreateTargetDataLayout(machine);
        let data_layout_str =
            CStr::from_ptr(llvm_sys::target::LLVMCopyStringRepOfTargetData(data_layout));

        LLVMDisposeTargetData(data_layout);
        LLVMDisposeTargetMachine(machine);
        LLVMDisposeMessage(target_triple);
        LLVMDisposeMessage(target_cpu);
        LLVMDisposeMessage(target_cpu_features);

        Ok(data_layout_str.to_string_lossy().into_owned())
    }
}
