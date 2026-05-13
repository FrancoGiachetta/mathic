
#include <mlir/IR/Builders.h>

#include "Dialect/Symbolic/IR/SymbolicDialect.h"
#include "Dialect/Symbolic/IR/SymbolicOps.h"
#include "Dialect/Symbolic/IR/SymbolicTypes.h"

#include "Dialect/Symbolic/IR/SymbolicDialect.cpp.inc"
#define GET_TYPEDEF_CLASSES
#include "Dialect/Symbolic/IR/SymbolicTypes.cpp.inc"
#define GET_OP_CLASSES
#include "Dialect/Symbolic/IR/SymbolicOps.cpp.inc"

namespace mlir
{
namespace symbolic
{
void SymbolicDialect::initialize()
{
    addTypes<
#define GET_TYPEDEF_LIST
#include "Dialect/Symbolic/IR/SymbolicTypes.cpp.inc"
        >();
    addOperations<
#define GET_OP_LIST
#include "Dialect/Symbolic/IR/SymbolicOps.cpp.inc"
        >();
}
} // namespace symbolic
} // namespace mlir
