#ifndef DIALECT_SYMBOLIC_CONVERSIONS_ARITH_PASSES_
#define DIALECT_SYMBOLIC_CONVERSIONS_ARITH_PASSES_

include "mlir/Pass/PassBase.td"

def SymbolicToArith : Pass<"symbolic-to-arith"> {
    let summary = "Convert to Arith dialect";
    let description = [{
        Converts primitive symbolic operations (like add, sub, div, mul) to
        their equivalent in the Arith dialect
    }];
    let dependentDialects = [
        "mlir::symbolic::SymbolicDialect",
        "mlir::func::FuncDialect",
        "mlir::arith::ArithDialect"
    ];
}

#endif // DIALECT_SYMBOLIC_CONVERSIONS_ARITH_PASSES_
