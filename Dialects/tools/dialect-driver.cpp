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
}

int main(int argc, char **argv)
{
    mlir::DialectRegistry registry;

    registry.insert<mlir::symbolic::SymbolicDialect>();

    mlir::registerAllDialects(registry);

    mlir::registerAllPasses();

    mlir::PassPipelineRegistration<>("symbolic-extract-eval", "Run pass to extract eval operations into functions",
                                     extractEvalOpsPipeline);
    return mlir::asMainReturnCode(mlir::MlirOptMain(argc, argv, "Dialect Driver", registry));
}
