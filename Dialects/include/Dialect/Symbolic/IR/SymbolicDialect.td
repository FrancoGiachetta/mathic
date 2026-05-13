#ifndef SYMBOLIC_DIALECT
#define SYMBOLIC_DIALECT

include "mlir/IR/OpBase.td"

def Symbolic_Dialect : Dialect {
    let name = "symbolic";
    let cppNamespace = "::mlir::symbolic";
    let description = [{
        Symbolic algebra dialect for Mathic compiler.
    }];

    // let useDefaultTypePrinterParser = 1;
}

#endif // SYMBOLIC_DIALECT
