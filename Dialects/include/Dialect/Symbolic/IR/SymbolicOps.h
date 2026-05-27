#pragma once

#include "Dialect/Symbolic/IR/SymbolicDialect.h"
#include "Dialect/Symbolic/IR/SymbolicTypes.h"

#include <mlir/IR/BuiltinOps.h>
#include <mlir/IR/BuiltinTypes.h>
#include <mlir/IR/Dialect.h>
#include <mlir/Interfaces/InferTypeOpInterface.h>
#include <mlir/Interfaces/SideEffectInterfaces.h>

#define GET_OP_CLASSES
#include "Dialect/Symbolic/IR/SymbolicOps.h.inc"
