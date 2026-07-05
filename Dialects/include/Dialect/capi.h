#pragma once

#include <mlir-c/IR.h>

#include "Dialect/Symbolic/Conversions/SymbolicToArith/SymbolicToArith.capi.h.inc"
#include "Dialect/Symbolic/Transforms/Passes.capi.h.inc"

#ifdef __cplusplus
extern "C"
{
#endif

    MLIR_DECLARE_CAPI_DIALECT_REGISTRATION(Symbolic, symbolic);
    MLIR_CAPI_EXPORTED void mlirInsertSymbolicDialect(MlirDialectRegistry registry);
    MLIR_CAPI_EXPORTED MlirType getSymExprType(MlirContext ctx, MlirType innerType, bool isSigned);

#ifdef __cplusplus
}
#endif
