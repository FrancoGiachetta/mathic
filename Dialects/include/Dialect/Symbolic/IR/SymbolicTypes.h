#pragma once

#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/MLIRContext.h"
// Required because the .h.inc file refers to MLIR classes and does not itself
// have any includes.
#include <cstdint>
#include <llvm/ADT/TypeSwitch.h>
#include <mlir/IR/Builders.h>
#include <mlir/IR/DialectImplementation.h>

#define GET_TYPEDEF_CLASSES
#include "Dialect/Symbolic/IR/SymbolicTypes.h.inc"

namespace mlir
{
namespace symbolic
{
} // namespace symbolic
} // namespace mlir
