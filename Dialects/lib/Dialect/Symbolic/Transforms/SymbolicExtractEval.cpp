#include "Dialect/Symbolic/Transforms/SymbolicExtractEval.h"
#include "Dialect/Symbolic/IR/SymbolicOps.h"
#include "Dialect/Symbolic/IR/SymbolicTypes.h"
#include <cstdint>
#include <llvm/ADT/Hashing.h>
#include <llvm/Support/Casting.h>
#include <mlir/Dialect/Arith/IR/Arith.h>
#include <mlir/Dialect/Func/IR/FuncOps.h>
#include <mlir/IR/Block.h>
#include <mlir/IR/BuiltinAttributes.h>
#include <mlir/IR/BuiltinOps.h>
#include <mlir/IR/BuiltinTypes.h>
#include <mlir/IR/MLIRContext.h>
#include <mlir/IR/OperationSupport.h>
#include <mlir/IR/PatternMatch.h>
#include <mlir/IR/Value.h>
#include <mlir/Support/LLVM.h>
#include <mlir/Transforms/GreedyPatternRewriteDriver.h>
#include <optional>
#include <string>
#include <utility>

namespace
{
using namespace mlir;
using namespace symbolic;

static Value cloneSymbolicOperationsIntoFunction(Value val, OpBuilder &builder, DenseMap<Value, Value> &valueMap);

/// Creates the hash of an expression.
///
/// This hash is constructed be traversing the expression and calculating the
/// expressions' hashes as well.
static std::optional<llvm::hash_code> getExpressionHash(mlir::Value value)
{
    mlir::Operation *op = value.getDefiningOp();

    if (!op)
        return std::nullopt;

    if (llvm::isa<arith::ConstantOp>(op))
        return llvm::hash_combine(llvm::cast<arith::ConstantOp>(op).getValue());

    if (llvm::isa<symbolic::SymOp>(op))
        return llvm::hash_combine(llvm::dyn_cast<symbolic::SymOp>(op).getName());

    if (llvm::isa<AddOp, SubOp, MulOp, DivOp>(op))
    {
        std::optional<llvm::hash_code> lhs = getExpressionHash(op->getOperand(0));
        if (!lhs)
            return std::nullopt;

        std::optional<llvm::hash_code> rhs = getExpressionHash(op->getOperand(1));
        if (!rhs)
            return std::nullopt;

        return llvm::hash_combine(op, lhs, rhs);
    }
}

static Value cloneBinop(Operation *op, Type opExprType, OpBuilder &builder, DenseMap<Value, Value> &valueMap)
{
    Value lhs = cloneSymbolicOperationsIntoFunction(op->getOperand(0), builder, valueMap);
    Value rhs = cloneSymbolicOperationsIntoFunction(op->getOperand(1), builder, valueMap);

    using namespace symbolic;
    if (isa<AddOp>(op))
        return builder.create<AddOp>(op->getLoc(), opExprType, lhs, rhs);
    if (isa<SubOp>(op))
        return builder.create<SubOp>(op->getLoc(), opExprType, lhs, rhs);
    if (isa<MulOp>(op))
        return builder.create<MulOp>(op->getLoc(), opExprType, lhs, rhs);
    return builder.create<DivOp>(op->getLoc(), opExprType, lhs, rhs);
}

/// Helper function to move the symbolic operations of an expression into its
/// associated eval function.
static Value cloneSymbolicOperationsIntoFunction(Value val, OpBuilder &builder, DenseMap<Value, Value> &valueMap)
{
    // Already cloned — return cached
    auto it = valueMap.find(val);

    if (it != valueMap.end())
        return it->second;

    Operation *op = val.getDefiningOp();

    if (!op)
        return Value();

    Type opExprType = op->getResult(0).getType();

    if (llvm::isa<arith::ConstantOp>(op))
        return builder.create<arith::ConstantOp>(op->getLoc(), llvm::cast<arith::ConstantOp>(op).getValue());

    if (llvm::isa<SymOp>(op))
    {
        // Clone SymOp as-is — same name, no substitution
        Value result = builder.create<SymOp>(op->getLoc(), opExprType, cast<SymOp>(op).getNameAttr());
        valueMap[val] = result;
        return result;
    }

    if (llvm::isa<AddOp, SubOp, MulOp, DivOp>(op))
    {
        Value result = cloneBinop(op, opExprType, builder, valueMap);
        valueMap[val] = result;
        return result;
    }

    return Value();
}
} // namespace

namespace mlir
{
namespace symbolic
{
#define GEN_PASS_DEF_SYMBOLICEXTRACTEVAL
#include "Dialect/Symbolic/Transforms/Passes.h.inc"

struct EvalToFuncState
{
    DenseMap<uint32_t, SymbolRefAttr> funcs;
};

struct EvalOpToFuncPattern : public OpRewritePattern<EvalOp>
{
    mutable EvalToFuncState state;

    EvalOpToFuncPattern(MLIRContext *ctx, EvalToFuncState &initState)
        : OpRewritePattern<EvalOp>(ctx), state(std::move(initState))
    {
    }

    LogicalResult matchAndRewrite(EvalOp op, PatternRewriter &rewriter) const override
    {
        std::optional<llvm::hash_code> evalOpHash = getExpressionHash(op.getExpr());

        if (!evalOpHash)
            return failure();

        SymbolRefAttr fnName;
        auto func = state.funcs.find(*evalOpHash);

        // Don't create a new function if the expression being evaluated
        // already has its associated function.
        if (func != state.funcs.end())
            fnName = func->second;
        else
        {
            ModuleOp module = op->getParentOfType<ModuleOp>();

            rewriter.setInsertionPointToStart(module.getBody());

            fnName = SymbolRefAttr::get(op.getContext(), "__eval_op_" + std::to_string(evalOpHash.value()));

            SymExprType exprTy = llvm::cast<SymExprType>(op.getExpr().getType());
            Type innerTy = exprTy.getInnerType();

            FunctionType fnType = rewriter.getFunctionType(innerTy, op.getExpr().getType());
            func::FuncOp fnOp = rewriter.create<func::FuncOp>(op.getLoc(), fnName.getLeafReference(), fnType);

            fnOp.setPrivate();

            Block *fnEntryBLock = fnOp.addEntryBlock();

            rewriter.setInsertionPointToStart(fnEntryBLock);

            DenseMap<Value, Value> valueMap;

            // Move the symbolic operations to the new function.
            Value result = cloneSymbolicOperationsIntoFunction(op.getExpr(), rewriter, valueMap);

            rewriter.create<func::ReturnOp>(op.getLoc(), result);
            rewriter.setInsertionPoint(op);

            state.funcs[static_cast<int32_t>(*evalOpHash)] = fnName;
        }

        func::CallOp call = rewriter.create<func::CallOp>(op.getLoc(), fnName, op.getExpr().getType(), op.getValue());

        // In this stage, the function actually return a !symbolic.expr. Since
        // we need the result to be numeric in the future, this operation acts
        // as placeholder to legalize the ir.
        mlir::UnrealizedConversionCastOp cast =
            rewriter.create<UnrealizedConversionCastOp>(op.getLoc(), op.getResult().getType(), call.getResult(0));

        rewriter.replaceOp(op, cast.getResult(0));

        return success();
    }
};

/// Pass to create a function associated to an eval operation.
///
/// The operations that conform the expression to evaluate are move into this
/// function. This is to make it possible to evaluate an expression more than
//  once without cloned operations.
struct SymbolicExtractEval : impl::SymbolicExtractEvalBase<SymbolicExtractEval>
{
    using SymbolicExtractEvalBase::SymbolicExtractEvalBase;

    void runOnOperation() override
    {
        mlir::RewritePatternSet patterns(&getContext());
        EvalToFuncState state;

        patterns.add<EvalOpToFuncPattern>(&getContext(), state);

        (void)applyPatternsGreedily(getOperation(), std::move(patterns));
    }
};
} // namespace symbolic
} // namespace mlir
