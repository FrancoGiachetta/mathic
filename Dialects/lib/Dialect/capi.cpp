#include <mlir-c/IR.h>
#include <mlir/CAPI/IR.h>
#include <mlir/CAPI/Pass.h>
#include <mlir/CAPI/Registration.h>
#include <mlir/IR/BuiltinTypes.h>
#include <mlir/IR/DialectRegistry.h>

#include "Dialect/Symbolic/Conversions/SymbolicToArith/SymbolicToArith.h"
#include "Dialect/Symbolic/IR/SymbolicDialect.h"
#include "Dialect/Symbolic/IR/SymbolicTypes.h"
#include "Dialect/Symbolic/Transforms/Passes.h"
#include "Dialect/capi.h"

MLIR_DEFINE_CAPI_DIALECT_REGISTRATION(Symbolic, symbolic, mlir::symbolic::SymbolicDialect)

extern "C" void mlirInsertSymbolicDialect(MlirDialectRegistry registry)
{
    mlirDialectHandleInsertDialect(mlirGetDialectHandle__symbolic__(), registry);
}

MlirType getSymExprType(MlirContext ctx, MlirType innerType, bool isSigned)
{
    return wrap(
        mlir::symbolic::SymExprType::get(unwrap(ctx), llvm::cast<mlir::IntegerType>(unwrap(innerType)), isSigned));
}

using namespace mlir;

extern "C"
{

    MlirPass mlirCreateSymbolicExtractEval()
    {
        return wrap(mlir::symbolic::createSymbolicExtractEval().release());
    }

    void mlirRegisterSymbolicExtractEval()
    {
        mlir::symbolic::registerSymbolicExtractEval();
    }

    MlirPass mlirCreateSymbolicToArith()
    {
        return wrap(mlir::symbolic::createSymbolicToArith().release());
    }

    void mlirRegisterSymbolicToArith()
    {
        mlir::symbolic::registerSymbolicToArith();
    }
}
