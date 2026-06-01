#ifndef DIALECT_SYMBOLIC_PASSES_
#define DIALECT_SYMBOLIC_PASSES_

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

// def SymbolicToArith : Pass<"symbolic-to-arith"> {
//     let summary = "Convert to Arith dialect";
//     let description = [{
//         Converts primitive symbolic operations (like add, sub, div, mul) to
//         their equivalent in the Arith dialect
//     }];
//     let dependentDialects = [
//         "mlir::symbolic::SymbolicDialect",
//         "mlir::func::FuncDialect",
//         "mlir::arith::ArithDialect"
//     ];
// }

#endif // DIALECT_SYMBOLIC_PASSES_
