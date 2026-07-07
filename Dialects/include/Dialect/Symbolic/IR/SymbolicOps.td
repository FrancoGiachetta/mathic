#ifndef DIALECT_SYMBOLIC_OPS_
#define DIALECT_SYMBOLIC_OPS_

include "SymbolicDialect.td"
include "SymbolicTypes.td"
include "mlir/Interfaces/SideEffectInterfaces.td"
include "mlir/IR/CommonTypeConstraints.td"

class Symbolic_BinOp<string mnemonic, list<Trait> traits = []>
    : Op<Symbolic_Dialect, mnemonic, !listconcat([ Pure ], traits)>
{
    let arguments = (ins
        AnyTypeOf<[SymExpr, AnyInteger]>:$lhs,
        AnyTypeOf<[SymExpr, AnyInteger]>:$rhs
    );
    let results = (outs SymExpr:$res);
    let assemblyFormat = "$lhs `,` $rhs attr-dict `:` `(` qualified(type($lhs)) `,` qualified(type($rhs)) `)` `->` qualified(type($res))";
}

def Symbolic_AddOp : Symbolic_BinOp<"add", [Commutative]>
{
    let summary = "Addition of symbolic expressions";
}

def Symbolic_SubOp : Symbolic_BinOp<"sub">
{
    let summary = "Subtraction of symbolic expressions";
}

def Symbolic_MulOp : Symbolic_BinOp<"mul", [Commutative]>
{
    let summary = "Multiplication of symbolic expressions";
}

def Symbolic_DivOp : Symbolic_BinOp<"div">
{
    let summary = "Division of symbolic expressions";
}

def Symbolic_SymOp : Op<Symbolic_Dialect, "sym", [Pure]>
{
    let summary = "Introduce a symbolic variable";
    let arguments = (ins StrAttr:$name);
    let results   = (outs SymExpr:$res);
    let assemblyFormat = "$name attr-dict `:` qualified(type($res))";
}

def Symbolic_EvalOp : Op<Symbolic_Dialect, "eval", []>
{
    let summary = "Evaluate a symbolic expression substituting a variable with a concrete value";
    let arguments = (ins SymExpr:$expr, StrAttr:$sym, AnyInteger:$value);
    let results = (outs AnyInteger:$res);
    let assemblyFormat = "$expr `,` $sym `,` $value attr-dict `:` `(` qualified(type($expr)) `,` type($value) `)` `->` type($res)";
}

#endif // DIALECT_SYMBOLIC_OPS_
