#ifndef DIALECT_SYMBOLIC_OPS_
#define DIALECT_SYMBOLIC_OPS_

include "SymbolicDialect.td"
include "SymbolicTypes.td"
include "mlir/Interfaces/SideEffectInterfaces.td"

    class Symbolic_BinOp<string mnemonic> : Op<Symbolic_Dialect, mnemonic, [Pure, SameOperandsAndResultElementType]> {
        let arguments = (ins SymExpr : $lhs, SymExpr : $rhs);
        let results = (outs SymExpr : $res);
        let assemblyFormat = "$lhs `,` $rhs attr-dict `:` type($res)";
        let hasFolder = 1;
    }

def Symbolic_Add : Symbolic_BinOp<"add">
{
    let summary = "Addition operation between symbolic expressions";
}

def Symbolic_Sub : Symbolic_BinOp<"sub">
{
    let summary = "Substraction operation between symbolic expressions";
}

def Symbolic_Mul : Symbolic_BinOp<"mul">
{
    let summary = "Multiplication operation between symbolic expressions";
}

def Symbolic_Div : Symbolic_BinOp<"div">
{
    let summary = "Division operation between symbolic expressions";
}

#endif // DIALECT_SYMBOLIC_OPS_
