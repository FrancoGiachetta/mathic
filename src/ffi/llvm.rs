use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    ptr::{addr_of_mut, null_mut},
};

use llvm_sys::{
    LLVMModule,
    core::{LLVMContextCreate, LLVMDisposeMessage, LLVMDisposeModule},
    error::{LLVMDisposeErrorMessage, LLVMGetErrorMessage},
    orc2::{
        LLVMOrcCreateNewThreadSafeModule, LLVMOrcDisposeThreadSafeModule,
        LLVMOrcThreadSafeContextRef,
        lljit::{
            LLVMOrcCreateLLJIT, LLVMOrcCreateLLJITBuilder, LLVMOrcDisposeLLJIT,
            LLVMOrcLLJITAddLLVMIRModule, LLVMOrcLLJITGetMainJITDylib, LLVMOrcLLJITRef,
        },
    },
    prelude::LLVMContextRef,
    target::{
        LLVM_InitializeAllAsmPrinters, LLVM_InitializeAllTargetInfos, LLVM_InitializeAllTargetMCs,
        LLVM_InitializeAllTargets, LLVMCopyStringRepOfTargetData, LLVMDisposeTargetData,
    },
    target_machine::{
        LLVMCodeGenFileType, LLVMCodeGenOptLevel, LLVMCodeModel, LLVMCreateTargetDataLayout,
        LLVMCreateTargetMachine, LLVMDisposeTargetMachine, LLVMGetDefaultTargetTriple,
        LLVMGetHostCPUFeatures, LLVMGetHostCPUName, LLVMGetTargetFromTriple,
        LLVMOpaqueTargetMachine, LLVMRelocMode, LLVMTargetMachineEmitToFile, LLVMTargetRef,
    },
    transforms::pass_builder::{
        LLVMCreatePassBuilderOptions, LLVMDisposePassBuilderOptions, LLVMRunPasses,
    },
};
use melior::ir::Module;
use mlir_sys::mlirTranslateModuleToLLVMIR;

use crate::diagnostics::CodegenError;

pub fn initialize_llvm() {
    unsafe {
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmPrinters();
    }
}

pub fn lower_mlir_to_llvm(
    llvm_ctx: LLVMContextRef,
    module: &Module,
    opt_lvl: usize,
    dump_llvmir: bool,
) -> Result<*mut LLVMModule, CodegenError> {
    unsafe {
        let llvm_module =
            mlirTranslateModuleToLLVMIR(module.as_operation().to_raw(), llvm_ctx as *mut _)
                as *mut _;
        let machine = create_llvm_machine(opt_lvl)?;
        let pass_builder_opts = LLVMCreatePassBuilderOptions();

        let passes = CString::new(format!("default<O{opt_lvl}>")).unwrap();

        let passes_error = LLVMRunPasses(llvm_module, passes.as_ptr(), machine, pass_builder_opts);

        if !passes_error.is_null() {
            let error = LLVMGetErrorMessage(passes_error);
            let msg = CStr::from_ptr(error).to_string_lossy().into_owned();

            LLVMDisposeTargetMachine(machine);
            LLVMDisposeModule(llvm_module);
            LLVMDisposeErrorMessage(error);

            return Err(CodegenError::LLVMError(msg))?;
        }

        LLVMDisposePassBuilderOptions(pass_builder_opts);

        if dump_llvmir {
            let mut null = null_mut();
            let error_buffer = addr_of_mut!(null);
            let file_path = CString::new("llvm_dump.ll").unwrap();
            let ok = LLVMTargetMachineEmitToFile(
                machine,
                llvm_module,
                file_path.as_ptr(),
                LLVMCodeGenFileType::LLVMAssemblyFile,
                error_buffer,
            );

            if ok != 0 {
                let error = CStr::from_ptr(*error_buffer).to_string_lossy().into_owned();

                LLVMDisposeMessage(*error_buffer);
                LLVMDisposeTargetMachine(machine);
                LLVMDisposeModule(llvm_module);

                return Err(CodegenError::LLVMError(error))?;
            } else if !(*error_buffer).is_null() {
                LLVMDisposeMessage(*error_buffer);
            }
        }

        LLVMDisposeTargetMachine(machine);

        Ok(llvm_module)
    }
}

unsafe extern "C" {
    // llvm_sys does not provide the function in its api, so we need to declare it manually.
    fn LLVMOrcCreateNewThreadSafeContextFromLLVMContext(
        Ctx: LLVMContextRef,
    ) -> LLVMOrcThreadSafeContextRef;
}

pub fn create_llvm_jit(
    modules: &[Module],
    opt_lvl: usize,
    dump_llvm: bool,
) -> Result<LLVMOrcLLJITRef, CodegenError> {
    unsafe {
        let context = LLVMContextCreate();
        let tsm_context = LLVMOrcCreateNewThreadSafeContextFromLLVMContext(context);
        let mut tsms = vec![];

        for m in modules {
            tsms.push(LLVMOrcCreateNewThreadSafeModule(
                lower_mlir_to_llvm(context, m, opt_lvl, dump_llvm)?,
                tsm_context,
            ));
        }

        let builder = LLVMOrcCreateLLJITBuilder();
        let mut jit: MaybeUninit<LLVMOrcLLJITRef> = MaybeUninit::uninit();

        let err = LLVMOrcCreateLLJIT(jit.as_mut_ptr(), builder);

        if !err.is_null() {
            let error = LLVMGetErrorMessage(err);
            let msg = CStr::from_ptr(error).to_string_lossy().into_owned();

            LLVMDisposeErrorMessage(error);
            for tsm in tsms {
                LLVMOrcDisposeThreadSafeModule(tsm);
            }

            return Err(CodegenError::LLVMError(msg));
        }

        let jit = jit.assume_init();

        let dylib = LLVMOrcLLJITGetMainJITDylib(jit);

        for tsm in tsms {
            let err = LLVMOrcLLJITAddLLVMIRModule(jit, dylib, tsm);

            if !err.is_null() {
                let error = LLVMGetErrorMessage(err);
                let msg = CStr::from_ptr(error).to_string_lossy().into_owned();

                LLVMOrcDisposeLLJIT(jit);
                LLVMDisposeErrorMessage(error);

                return Err(CodegenError::LLVMError(msg));
            }
        }

        Ok(jit)
    }
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
pub fn get_data_layout_rep(opt_lvl: usize) -> Result<String, CodegenError> {
    unsafe {
        let machine = create_llvm_machine(opt_lvl)?;
        let data_layout = LLVMCreateTargetDataLayout(machine);
        let data_layout_str = CStr::from_ptr(LLVMCopyStringRepOfTargetData(data_layout));

        LLVMDisposeTargetData(data_layout);
        LLVMDisposeTargetMachine(machine);

        Ok(data_layout_str.to_string_lossy().into_owned())
    }
}

fn create_llvm_machine(opt_lvl: usize) -> Result<*mut LLVMOpaqueTargetMachine, CodegenError> {
    unsafe {
        let mut null = null_mut();
        let error_buffer = addr_of_mut!(null);

        let target_triple = LLVMGetDefaultTargetTriple();
        let target_cpu = LLVMGetHostCPUName();
        let target_cpu_features = LLVMGetHostCPUFeatures();

        let mut target: MaybeUninit<LLVMTargetRef> = MaybeUninit::uninit();

        if LLVMGetTargetFromTriple(target_triple, target.as_mut_ptr(), error_buffer) != 0 {
            let error = CStr::from_ptr(*error_buffer);
            let err = error.to_string_lossy().into_owned();
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
                0 => LLVMCodeGenOptLevel::LLVMCodeGenLevelNone,
                1 => LLVMCodeGenOptLevel::LLVMCodeGenLevelLess,
                2 => LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
                _ => LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
            },
            LLVMRelocMode::LLVMRelocDefault,
            LLVMCodeModel::LLVMCodeModelDefault,
        ) as *mut _;

        LLVMDisposeMessage(target_triple);
        LLVMDisposeMessage(target_cpu);
        LLVMDisposeMessage(target_cpu_features);

        Ok(machine)
    }
}
