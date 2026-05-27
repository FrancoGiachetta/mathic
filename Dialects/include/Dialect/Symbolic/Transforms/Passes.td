#ifndef DIALECT_SYMBOLIC_PASSES_
#define DIALECT_SYMBOLIC_PASSES_

include "mlir/Pass/PassBase.td"

def SymbolicCanonicalize : Pass<"symbolic-canonicalize"> {
    let summary = "Canonicalize expressions";
    let description = [{
        Removes non primitive expressions such as derivatives, integrals
    }];
}

#endif // DIALECT_SYMBOLIC_PASSES_
