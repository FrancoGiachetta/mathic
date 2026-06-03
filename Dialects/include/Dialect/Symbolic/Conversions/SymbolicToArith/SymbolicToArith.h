#pragma once

#include <mlir/Dialect/Arith/IR/Arith.h>
#include <mlir/Dialect/Func/IR/FuncOps.h>
#include <mlir/Pass/Pass.h>

namespace mlir
{
namespace symbolic
{
#define GEN_PASS_DECL
#include "Dialect/Symbolic/Conversions/SymbolicToArith/SymbolicToArith.h.inc"

#define GEN_PASS_REGISTRATION
#include "Dialect/Symbolic/Conversions//SymbolicToArith/SymbolicToArith.h.inc"
} // namespace symbolic
} // namespace mlir
