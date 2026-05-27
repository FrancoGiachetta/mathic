#ifndef SYMBOLIC_DIALECT
#define SYMBOLIC_DIALECT

include "mlir/IR/DialectBase.td"

def Symbolic_Dialect : Dialect {
    let name = "symbolic";
    let cppNamespace = "::mlir::symbolic";
    let description = [{
        Symbolic algebra dialect for Mathic compiler.
    }];

    let useDefaultTypePrinterParser = 1;
    let usePropertiesForAttributes = 1;
}

#endif // SYMBOLIC_DIALECT
