#include "Dialect/Symbolic/IR/SymbolicOps.h"
#include "Dialect/Symbolic/Transforms/SymbolicCanonicalize.h"
#include "mlir/IR/MLIRContext.h"
#include "mlir/IR/PatternMatch.h"
#include "mlir/Support/LLVM.h"
#include <mlir/IR/OperationSupport.h>
#include <mlir/Transforms/GreedyPatternRewriteDriver.h>

namespace mlir
{
namespace symbolic
{

#define GEN_PASS_DEF_SYMBOLICCANONICALIZE
#include "Dialect/Symbolic/Transforms/Passes.h.inc"

/// Removes derivatives by solving them.
struct RemoveDerivatives : public OpRewritePattern<DiffOp>
{
    RemoveDerivatives(mlir::MLIRContext *ctx) : OpRewritePattern<DiffOp>(ctx)
    {
    }

    LogicalResult matchAndRewrite(DiffOp op, PatternRewriter &rewriter) const override
    {
        auto expr = op.getExpr();
        return success();
    }
};

/// Symbolic canonicalizer.
///
/// Brings an expression to its canonical form, which is made of primitive operations.
struct SymbolicCanonicalize : impl::SymbolicCanonicalizeBase<SymbolicCanonicalize>
{
    using SymbolicCanonicalizeBase::SymbolicCanonicalizeBase;

    void runOnOperation() override
    {
        mlir::RewritePatternSet patterns(&getContext());

        patterns.add<RemoveDerivatives>(&getContext());

        (void)applyPatternsGreedily(getOperation(), std::move(patterns));
    }
};
} // namespace symbolic
} // namespace mlir
