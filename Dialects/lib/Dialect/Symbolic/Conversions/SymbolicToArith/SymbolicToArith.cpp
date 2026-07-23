#include "Dialect/Symbolic/Conversions/SymbolicToArith/SymbolicToArith.h"
#include "Dialect/Symbolic/IR/SymbolicDialect.h"
#include "Dialect/Symbolic/IR/SymbolicOps.h"
#include "Dialect/Symbolic/IR/SymbolicTypes.h"
#include "llvm/Support/Casting.h"
#include <llvm/Support/LogicalResult.h>
#include <mlir/Dialect/Arith/IR/Arith.h>
#include <mlir/Dialect/Func/IR/FuncOps.h>
#include <mlir/Dialect/Func/Transforms/FuncConversions.h>
#include <mlir/IR/BuiltinTypes.h>
#include <mlir/IR/MLIRContext.h>
#include <mlir/IR/Types.h>
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
        /// For now, every !symbolic.expr is converted to float64.
        addConversion([](SymExprType exprTy) -> Type { return exprTy.getInnerType(); });
    }
};

BINARY_OP_CONVERTER(Add, AddIOp)
BINARY_OP_CONVERTER(Sub, SubIOp)
BINARY_OP_CONVERTER(Mul, MulIOp)
BINARY_OP_CONVERTER_SIG_UNSIG(Div, DivSIOp, DivUIOp)

/// Replace symbols witht the function's actual argument to be evaluated.
struct ConvertSym : public OpConversionPattern<symbolic::SymOp>
{
    ConvertSym(MLIRContext *ctx) : OpConversionPattern<symbolic::SymOp>(ctx)
    {
    }

    using OpConversionPattern::OpConversionPattern;

    llvm::LogicalResult matchAndRewrite(symbolic::SymOp op, OpAdaptor adaptor,
                                        ConversionPatternRewriter &rewriter) const override
    {
        auto func = op->getParentOfType<func::FuncOp>();
        if (!func || func.getNumArguments() < 1)
            return llvm::failure();

        rewriter.replaceOp(op, func.getArgument(0));
        return llvm::success();
    }
};

/// Get rid of UnrealizedConversionCast operations.
struct ConvertCast : public OpConversionPattern<UnrealizedConversionCastOp>
{
    using OpConversionPattern::OpConversionPattern;

    llvm::LogicalResult matchAndRewrite(UnrealizedConversionCastOp op, OpAdaptor adaptor,
                                        ConversionPatternRewriter &rewriter) const override
    {
        if (op->getNumOperands() != 1 || op->getNumResults() != 1)
            return llvm::failure();

        auto inTypes = adaptor.getOperands().getTypes();
        auto outTypes = op->getResultTypes();

        if (inTypes == outTypes)
        {
            rewriter.replaceOp(op, adaptor.getOperands());
            return llvm::success();
        }

        return llvm::failure();
    }
};

struct SymbolicToArith : impl::SymbolicToArithBase<SymbolicToArith>
{
    using SymbolicToArithBase::SymbolicToArithBase;

    void runOnOperation() override
    {
        MLIRContext *ctx = &getContext();
        Operation *module = getOperation();
        ConversionTarget target(*ctx);
        SymbolicToArithTypeConverter typeConverter(ctx);

        target.addLegalDialect<arith::ArithDialect>();
        // After this pass, there shouldn't be any reference to the symbolic
        // dialect.
        target.addIllegalDialect<SymbolicDialect>();

        mlir::RewritePatternSet patterns(&getContext());

        patterns.add<ConvertAdd, ConvertSub, ConvertMul, ConvertDiv, ConvertSym, ConvertCast>(typeConverter, ctx);

        // Propagate the type convertions across functions' signatures.
        populateFunctionOpInterfaceTypeConversionPattern<func::FuncOp>(patterns, typeConverter);
        target.addDynamicallyLegalOp<func::FuncOp>([&](func::FuncOp op) {
            return typeConverter.isSignatureLegal(op.getFunctionType()) && typeConverter.isLegal(&op.getBody());
        });

        // Propagate the type convertions across call operations.
        populateCallOpTypeConversionPattern(patterns, typeConverter);
        target.addDynamicallyLegalOp<func::CallOp>([&](func::CallOp op) { return typeConverter.isLegal(op); });

        // Propagate the type convertions across return operations.
        populateReturnOpTypeConversionPattern(patterns, typeConverter);
        target.addDynamicallyLegalOp<func::ReturnOp>([&](func::ReturnOp op) { return typeConverter.isLegal(op); });

        if (llvm::failed(applyPartialConversion(module, target, std::move(patterns))))
            signalPassFailure();
    }
};
} // namespace symbolic
} // namespace mlir
