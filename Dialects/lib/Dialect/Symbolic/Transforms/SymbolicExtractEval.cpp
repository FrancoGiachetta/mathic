#include "Dialect/Symbolic/Transforms/SymbolicExtractEval.h"
#include "Dialect/Symbolic/IR/SymbolicOps.h"
#include "Dialect/Symbolic/IR/SymbolicTypes.h"
#include <cstdint>
#include <llvm/ADT/Hashing.h>
#include <llvm/Support/Casting.h>
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

/// Creates the hash of an expression.
///
/// This hash is constructed be traversing the expression and calculating the
/// expressions' hashes as well.
static std::optional<llvm::hash_code> getExpressionHash(mlir::Value value)
{
    mlir::Operation *op = value.getDefiningOp();

    if (!op)
        return std::nullopt;

    if (llvm::isa<symbolic::SymOp>(op))
    {

        llvm::StringRef sym = llvm::dyn_cast<symbolic::SymOp>(op).getName();
        return llvm::hash_combine(sym);
    }

    if (llvm::isa<symbolic::AddOp>(op) || llvm::isa<symbolic::SubOp>(op) || llvm::isa<symbolic::MulOp>(op) ||
        llvm::isa<symbolic::DivOp>(op))
    {
        std::optional<llvm::hash_code> lhs = getExpressionHash(op->getOperand(0));
        std::optional<llvm::hash_code> rhs = getExpressionHash(op->getOperand(1));

        if (!lhs || !rhs)
            return std::nullopt;

        return llvm::hash_combine(op, lhs, rhs);
    }
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
    Type opExprType = op->getResult(0).getType();

    if (!op)
        return Value();

    Value result;

    if (isa<symbolic::SymOp>(op))
    {
        // Clone SymOp as-is — same name, no substitution
        result = builder.create<symbolic::SymOp>(op->getLoc(), opExprType, dyn_cast<symbolic::SymOp>(op).getNameAttr());
    }
    else if (isa<symbolic::AddOp>(op))
    {
        Value lhs = cloneSymbolicOperationsIntoFunction(op->getOperand(0), builder, valueMap);
        Value rhs = cloneSymbolicOperationsIntoFunction(op->getOperand(1), builder, valueMap);
        result = builder.create<symbolic::AddOp>(op->getLoc(), opExprType, lhs, rhs);
    }
    else if (isa<symbolic::SubOp>(op))
    {
        Value lhs = cloneSymbolicOperationsIntoFunction(op->getOperand(0), builder, valueMap);
        Value rhs = cloneSymbolicOperationsIntoFunction(op->getOperand(1), builder, valueMap);
        result = builder.create<symbolic::SubOp>(op->getLoc(), opExprType, lhs, rhs);
    }
    else if (isa<symbolic::MulOp>(op))
    {
        Value lhs = cloneSymbolicOperationsIntoFunction(op->getOperand(0), builder, valueMap);
        Value rhs = cloneSymbolicOperationsIntoFunction(op->getOperand(1), builder, valueMap);
        result = builder.create<symbolic::MulOp>(op->getLoc(), opExprType, lhs, rhs);
    }
    else if (isa<symbolic::DivOp>(op))
    {
        Value lhs = cloneSymbolicOperationsIntoFunction(op->getOperand(0), builder, valueMap);
        Value rhs = cloneSymbolicOperationsIntoFunction(op->getOperand(1), builder, valueMap);
        result = builder.create<symbolic::DivOp>(op->getLoc(), opExprType, lhs, rhs);
    }
    else
    {
        return Value();
    }

    valueMap[val] = result;

    return result;
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

    EvalOpToFuncPattern(MLIRContext *ctx, EvalToFuncState &state) : OpRewritePattern<EvalOp>(ctx), state(state)
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
