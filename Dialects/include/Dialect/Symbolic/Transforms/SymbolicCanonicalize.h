#pragma once

#include <mlir/Pass/Pass.h>

namespace mlir
{
namespace symbolic
{
#define GEN_PASS_DECL_SYMBOLICCANONICALIZE
#include "Dialect/Symbolic/Transforms/Passes.h.inc"
} // namespace symbolic
} // namespace mlir
