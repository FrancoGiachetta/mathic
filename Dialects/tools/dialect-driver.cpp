#include "Dialect/Symbolic/Conversions/SymbolicToArith/SymbolicToArith.h"
#include "Dialect/Symbolic/IR/SymbolicDialect.h"
#include "Dialect/Symbolic/Transforms/SymbolicExtractEval.h"
#include <mlir/InitAllDialects.h>
#include <mlir/InitAllPasses.h>
#include <mlir/Pass/PassManager.h>
#include <mlir/Pass/PassRegistry.h>
#include <mlir/Tools/mlir-opt/MlirOptMain.h>
#include <mlir/Transforms/Passes.h>

void extractEvalOpsPipeline(mlir::OpPassManager &manager)
{
    manager.addPass(mlir::symbolic::createSymbolicExtractEval());
    manager.addPass(mlir::symbolic::createSymbolicToArith());
}

int main(int argc, char **argv)
{
    mlir::DialectRegistry registry;

    registry.insert<mlir::symbolic::SymbolicDialect>();

    mlir::registerAllDialects(registry);

    mlir::registerAllPasses();

    mlir::PassPipelineRegistration<>("symbolic-to-arith", "Run pass to pass to convert symbolic dialect to arith",
                                     extractEvalOpsPipeline);
    return mlir::asMainReturnCode(mlir::MlirOptMain(argc, argv, "Dialect Driver", registry));
}
