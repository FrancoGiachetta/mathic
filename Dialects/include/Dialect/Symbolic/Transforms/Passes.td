#ifndef DIALECT_SYMBOLIC_TRANSFORMS_PASSES_
#define DIALECT_SYMBOLIC_TRANSFORMS_PASSES_

include "mlir/Pass/PassBase.td"

def SymbolicExtractEval : Pass<"symbolic-extract-eval"> {
    let summary = "Extract the eval operation into a function";
    let description = [{
        Finds symbolic.eval operations, walks their expression DAG, and
        extracts each unique expression into a private function. Replaces eval
        with a func.call.
    }];
    let dependentDialects = [
        "mlir::symbolic::SymbolicDialect",
        "mlir::func::FuncDialect"
    ];
}

#endif // DIALECT_SYMBOLIC_TRANSFORMS_PASSES_
