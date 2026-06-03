#include "Dialect/Symbolic/Conversions/SymbolicToArith/SymbolicToArith.h"
#include "Dialect/Symbolic/IR/SymbolicDialect.h"
#include "Dialect/Symbolic/IR/SymbolicOps.h"
#include "Dialect/Symbolic/IR/SymbolicTypes.h"
#include "mlir/Dialect/Arith/IR/Arith.h"
#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/MLIRContext.h"
#include "mlir/IR/Types.h"
#include "llvm/Support/LogicalResult.h"
#include <mlir/Transforms/DialectConversion.h>

namespace mlir
{
namespace symbolic
{
#define GEN_PASS_DEF_SYMBOLICTOARITH
#include "Dialect/Symbolic/Conversions/SymbolicToArith/SymbolicToArith.h.inc"

class SymbolicToArithTypeConverter : public TypeConverter
{
  public:
    SymbolicToArithTypeConverter(MLIRContext *ctx)
    {
        addConversion([](Type ty) { return ty; });
        addConversion([ctx](SymExprType exprTy) -> Type { return Float64Type::get(ctx); });
    }
};

struct ConvertAdd : public OpConversionPattern<symbolic::AddOp>
{
    ConvertAdd(MLIRContext *ctx) : OpConversionPattern<symbolic::AddOp>(ctx)
    {
    }

    using OpConversionPattern::OpConversionPattern;

    llvm::LogicalResult matchAndRewrite(symbolic::AddOp op, OpAdaptor adaptor,
                                        ConversionPatternRewriter &rewriter) const override
    {
        arith::AddFOp addOp = rewriter.create<arith::AddFOp>(op.getLoc(), adaptor.getLhs(), adaptor.getRhs());

        rewriter.replaceOp(op.getOperation(), addOp);

        return llvm::success();
    }
};

struct SymbolicToArith : impl::SymbolicToArithBase<SymbolicToArith>
{
    using SymbolicToArithBase::SymbolicToArithBase;

    void runOnOperation() override
    {
        MLIRContext *ctx = &getContext();
        auto *module = getOperation();
        ConversionTarget target(*ctx);
        SymbolicToArithTypeConverter typeConverter(ctx);

        target.addLegalDialect<arith::ArithDialect>();
        target.addLegalDialect<SymbolicDialect>();

        mlir::RewritePatternSet patterns(&getContext());

        patterns.add<ConvertAdd>(typeConverter, ctx);

        if (llvm::failed(applyPartialConversion(module, target, std::move(patterns))))
            signalPassFailure();
    }
};
} // namespace symbolic
} // namespace mlir
