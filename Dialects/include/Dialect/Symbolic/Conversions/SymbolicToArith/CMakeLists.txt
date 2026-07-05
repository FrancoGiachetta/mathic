set(LLVM_TARGET_DEFINITIONS SymbolicToArith.td)

mlir_tablegen(SymbolicToArith.h.inc -gen-pass-decls -name SymbolicToArith)
mlir_tablegen(SymbolicToArith.capi.h.inc -gen-pass-capi-header -name SymbolicToArith)
mlir_tablegen(SymbolicToArith.capi.cpp.inc -gen-pass-capi-impl -name SymbolicToArith)

add_dependencies(mlir-headers MLIRSymbolicDialectIncGen)
add_dependencies(mlir-headers MLIRSymbolicOpsIncGen)
add_dependencies(mlir-headers MLIRSymbolicTypesIncGen)
add_public_tablegen_target(MLIRSymbolicToArithPassIncGen)
