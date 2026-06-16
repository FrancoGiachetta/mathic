#include <mlir-c/IR.h>
#include <mlir/CAPI/IR.h>
#include <mlir/IR/BuiltinTypes.h>
#include <mlir/IR/DialectRegistry.h>

#include "Dialect/Symbolic/IR/SymbolicDialect.h"
#include "Dialect/Symbolic/IR/SymbolicTypes.h"
#include "Dialect/capi.h"

namespace
{
static void CPPregisterSymbolicDialect(mlir::DialectRegistry &registry)
{
    registry.insert<mlir::symbolic::SymbolicDialect>();
}
} // namespace

void registerSymbolicDialect(MlirDialectRegistry registry)
{
    CPPregisterSymbolicDialect(*unwrap(registry));
}

MlirType getSymExprType(MlirContext ctx, MlirType innerType, bool isSigned)
{
    return wrap(
        mlir::symbolic::SymExprType::get(unwrap(ctx), llvm::cast<mlir::IntegerType>(unwrap(innerType)), isSigned));
}
