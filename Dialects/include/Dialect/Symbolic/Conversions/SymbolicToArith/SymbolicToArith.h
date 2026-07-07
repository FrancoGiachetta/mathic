#pragma once

#include <mlir/Dialect/Arith/IR/Arith.h>
#include <mlir/Dialect/Func/IR/FuncOps.h>
#include <mlir/Pass/Pass.h>

#define BINARY_OP_CONVERTER(SYM_OP, ARITH_OP)                                                                          \
    struct Convert##SYM_OP : public OpConversionPattern<symbolic::SYM_OP##Op>                                          \
    {                                                                                                                  \
        using OpConversionPattern::OpConversionPattern;                                                                \
                                                                                                                       \
        llvm::LogicalResult matchAndRewrite(symbolic::SYM_OP##Op op, OpAdaptor adaptor,                                \
                                            ConversionPatternRewriter &rewriter) const override                        \
        {                                                                                                              \
            auto newOp = arith::ARITH_OP::create(rewriter, op.getLoc(), adaptor.getLhs(), adaptor.getRhs());           \
            rewriter.replaceOp(op.getOperation(), newOp);                                                              \
            return llvm::success();                                                                                    \
        }                                                                                                              \
    };

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
