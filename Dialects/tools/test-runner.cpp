#include <mlir/InitAllDialects.h>
#include <mlir/InitAllPasses.h>
#include <mlir/Tools/mlir-opt/MlirOptMain.h>

#include "Dialect/Symbolic/IR/SymbolicDialect.h"

int main(int argc, char **argv)
{
    mlir::DialectRegistry registry;

    registry.insert<mlir::symbolic::SymbolicDialect>();

    mlir::registerAllDialects(registry);
    mlir::registerAllPasses();

    return mlir::asMainReturnCode(mlir::MlirOptMain(argc, argv, "Dialect Driver", registry));
}
