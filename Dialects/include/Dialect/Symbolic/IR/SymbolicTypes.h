#pragma once

#include "Dialect/Symbolic/IR/SymbolicDialect.h"
// Required because the .h.inc file refers to MLIR classes and does not itself
// have any includes.
#include <llvm/ADT/TypeSwitch.h>
#include <mlir/IR/DialectImplementation.h>
#include <mlir/IR/Builders.h>

#define GET_TYPEDEF_CLASSES
#include "Dialect/Symbolic/IR/SymbolicTypes.h.inc"
