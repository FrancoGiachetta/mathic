#ifndef SYMBOLIC_OPS
#define SYMBOLIC_OPS

include "SymbolicDialect.td"

class Symbolic_Op<string mnemonic, list<Trait> traits = []> :
    Op<Symbolic_Dialect, mnemonic, traits>;

#endif // SYMBOLIC_OPS
