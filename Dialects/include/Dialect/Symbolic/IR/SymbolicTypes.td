#ifndef DIALECT_SYMBOLIC_TYPES_
#define DIALECT_SYMBOLIC_TYPES_

include "SymbolicDialect.td"
include "mlir/IR/AttrTypeBase.td"

    class Symbolic_Type<string name, string typeMnemonic, list<Trait> traits = []>
    : TypeDef<Symbolic_Dialect, name, traits>
{
    let mnemonic = typeMnemonic;
}

def SymExpr : Symbolic_Type<"SymExpr", "expr">
{
    let summary = "A symbolic expression";
    let parameters = (ins "mlir::IntegerType":$innerType, "bool":$isSigned);
    let assemblyFormat = "`<` $innerType `,` struct($isSigned) `>`";
}

#endif // DIALECT_SYMBOLIC_TYPES_
