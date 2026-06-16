#pragma once

#include <mlir-c/IR.h>

#ifdef __cplusplus
extern "C"
{
#endif

    MLIR_CAPI_EXPORTED void registerSymbolicDialect(MlirDialectRegistry registry);
    MLIR_CAPI_EXPORTED MlirType getSymExprType(MlirContext ctx, MlirType innerType, bool isSigned);

#ifdef __cplusplus
}
#endif
