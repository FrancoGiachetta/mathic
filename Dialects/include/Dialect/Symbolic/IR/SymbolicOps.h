#pragma once

#include "SymbolicDialect.h"
#include "SymbolicTypes.h"
#include <mlir/IR/BuiltinOps.h>
#include <mlir/IR/BuiltinTypes.h>
#include <mlir/IR/Dialect.h>
#include <mlir/Interfaces/InferTypeOpInterface.h>

#define GET_OP_CLASSES
#include "Dialect/Symbolic/IR/SymbolicOps.h.inc"
