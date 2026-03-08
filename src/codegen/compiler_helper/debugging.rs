use std::collections::HashSet;

use melior::{
    Context,
    dialect::{llvm, ods},
    helpers::{ArithBlockExt, BuiltinBlockExt, LlvmBlockExt},
    ir::{
        Attribute, Block, BlockLike, Location, Module, Region, Value,
        attribute::{FlatSymbolRefAttribute, StringAttribute, TypeAttribute},
        operation::OperationBuilder,
        r#type::IntegerType,
    },
};

use crate::{codegen::compiler_helper::build_llvm_indirect_call, diagnostics::CodegenError};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum DebugBinding {
    PrintNumber,
    PrintPtr,
    PrintStr,
}

impl DebugBinding {
    fn symbol(&self) -> &'static str {
        match self {
            DebugBinding::PrintNumber => "mathic__debug__print_number_impl",
            DebugBinding::PrintPtr => "mathic__debug__print_ptr",
            DebugBinding::PrintStr => "mathic__debug__print_str",
        }
    }

    fn func_ptr(&self) -> *const () {
        match self {
            DebugBinding::PrintNumber => debug_utils_runtime::print_number_impl as *const (),
            DebugBinding::PrintPtr => debug_utils_runtime::print_ptr as *const (),
            DebugBinding::PrintStr => debug_utils_runtime::print_str as *const (),
        }
    }
}

pub struct DebugUtils {
    active_map: HashSet<DebugBinding>,
}

impl DebugUtils {
    pub fn new() -> Self {
        Self {
            active_map: Default::default(),
        }
    }

    fn build_function<'ctx, 'func>(
        &mut self,
        ctx: &'ctx Context,
        module: &Module,
        block: &'func Block<'ctx>,
        binding: DebugBinding,
    ) -> Result<Value<'ctx, 'func>, CodegenError> {
        let location = Location::unknown(ctx);
        let ptr_ty = llvm::r#type::pointer(ctx, 0);

        if self.active_map.insert(binding) {
            module.body().append_operation(
                ods::llvm::mlir_global(
                    ctx,
                    Region::new(),
                    TypeAttribute::new(llvm::r#type::pointer(ctx, 0)),
                    StringAttribute::new(ctx, binding.symbol()),
                    Attribute::parse(ctx, "#llvm.linkage<weak>").ok_or(
                        melior::Error::AttributeParse(
                            "could not parse linkage attribute".to_string(),
                        ),
                    )?,
                    Location::unknown(ctx),
                )
                .into(),
            );
        }

        let sym_addr = block.append_op_result(
            ods::llvm::mlir_addressof(
                ctx,
                ptr_ty,
                FlatSymbolRefAttribute::new(ctx, binding.symbol()),
                location,
            )
            .into(),
        )?;

        Ok(block.load(ctx, location, sym_addr, ptr_ty)?)
    }

    pub fn debug_breakpoint_trap(
        &self,
        block: &Block,
        location: Location,
    ) -> Result<(), CodegenError> {
        block.append_operation(OperationBuilder::new("llvm.intr.debugtrap", location).build()?);

        Ok(())
    }

    pub fn debug_print(
        &mut self,
        ctx: &Context,
        module: &Module,
        block: &Block,
        message: &str,
        location: Location,
    ) -> Result<(), CodegenError> {
        let ty = llvm::r#type::array(IntegerType::new(ctx, 8).into(), message.len() as u32);

        let ptr = block.alloca1(ctx, location, ty, 8)?;

        let msg = block.append_op_result(
            ods::llvm::mlir_constant(
                ctx,
                llvm::r#type::array(IntegerType::new(ctx, 8).into(), message.len() as u32),
                StringAttribute::new(ctx, message).into(),
                location,
            )
            .into(),
        )?;

        block.store(ctx, location, ptr, msg)?;

        let len = block.const_int(ctx, location, message.len(), 64)?;

        self.print_str(ctx, module, block, ptr, len)
    }

    pub fn print_str(
        &mut self,
        ctx: &Context,
        module: &Module,
        block: &Block,
        ptr: Value,
        len: Value,
    ) -> Result<(), CodegenError> {
        let func_ptr = self.build_function(ctx, module, block, DebugBinding::PrintStr)?;

        block.append_operation(build_llvm_indirect_call(ctx, &[func_ptr, ptr, len], &[])?);

        Ok(())
    }

    pub fn print_ptr(
        &mut self,
        ctx: &Context,
        module: &Module,
        block: &Block,
        ptr: Value,
    ) -> Result<(), CodegenError> {
        let func_ptr = self.build_function(ctx, module, block, DebugBinding::PrintPtr)?;

        block.append_operation(build_llvm_indirect_call(ctx, &[func_ptr, ptr], &[])?);

        Ok(())
    }

    pub fn print_number(
        &mut self,
        ctx: &Context,
        module: &Module,
        block: &Block,
        val: Value,
    ) -> Result<(), CodegenError> {
        let func_ptr = self.build_function(ctx, module, block, DebugBinding::PrintNumber)?;

        block.append_operation(build_llvm_indirect_call(ctx, &[func_ptr, val], &[])?);

        Ok(())
    }
}

pub mod debug_utils_runtime {
    use crate::codegen::compiler_helper::debugging::DebugBinding;

    pub fn setup(find_symbol_callback: impl Fn(&str) -> Option<*mut ()>) {
        for b in [
            DebugBinding::PrintNumber,
            DebugBinding::PrintPtr,
            DebugBinding::PrintStr,
        ]
        .iter()
        {
            if let Some(p) = find_symbol_callback(b.symbol()) {
                let p = p.cast::<*const ()>();

                unsafe {
                    *p = b.func_ptr();
                }
            }
        }
    }

    pub extern "C" fn print_number_impl(value: u128) {
        println!("DEBUG PRINT: {value}");
    }

    pub extern "C" fn print_ptr(value: *const ()) {
        println!("DEBUG PRINT: {value:018x?}");
    }

    pub extern "C" fn print_str(value: *const std::ffi::c_char, len: u64) {
        let str_slice = unsafe { std::slice::from_raw_parts(value as *const u8, len as usize) };
        println!("DEBUG PRINT: {}", str::from_utf8(str_slice).unwrap());
    }
}
