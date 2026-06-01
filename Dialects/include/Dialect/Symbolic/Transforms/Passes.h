#pragma once

#include "Dialect/Symbolic/Transforms/SymbolicExtractEval.h"
#include "Dialect/Symbolic/Transforms/SymbolicToArith.h"

namespace mlir
{
namespace symbolic
{
#define GEN_PASS_REGISTRATION
#include "Dialect/Symbolic/Transforms/Passes.h.inc"
} // namespace symbolic
} // namespace mlir
