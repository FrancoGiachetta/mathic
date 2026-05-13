set(LLVM_TARGET_DEFINITIONS SymbolicDialect.td)
mlir_tablegen(SymbolicDialect.h.inc   -gen-dialect-decls -dialect=symbolic)
mlir_tablegen(SymbolicDialect.cpp.inc -gen-dialect-defs -dialect=symbolic)
add_public_tablegen_target(MLIRSymbolicDialectIncGen)

# Ops
set(LLVM_TARGET_DEFINITIONS SymbolicOps.td)
mlir_tablegen(SymbolicOps.h.inc   -gen-op-decls)
mlir_tablegen(SymbolicOps.cpp.inc -gen-op-defs)
add_public_tablegen_target(MLIRSymbolicOpsIncGen)

# Types
set(LLVM_TARGET_DEFINITIONS SymbolicTypes.td)
mlir_tablegen(SymbolicTypes.h.inc   -gen-typedef-decls -typedefs-dialect=symbolic)
mlir_tablegen(SymbolicTypes.cpp.inc -gen-typedef-defs -typedefs-dialect=symbolic)
add_public_tablegen_target(MLIRSymbolicTypesIncGen)

# Umbrella target for everything generated from this dialect
add_dependencies(mlir-headers MLIRSymbolicDialectIncGen MLIRSymbolicOpsIncGen MLIRSymbolicTypesIncGen)
