#pragma once

#include <mlir/Dialect/Func/IR/FuncOps.h>
#include <mlir/Pass/Pass.h>

namespace mlir
{
namespace symbolic
{
#define GEN_PASS_DECL_SYMBOLICEXTRACTEVAL
#include "Dialect/Symbolic/Transforms/Passes.h.inc"
} // namespace symbolic
} // namespace mlir
