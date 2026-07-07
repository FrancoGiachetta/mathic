#include "Dialect/Symbolic/IR/SymbolicOps.h"
#include "Dialect/Symbolic/IR/SymbolicTypes.h"
#include "Dialect/Symbolic/Transforms/SymbolicExtractEval.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include <cstdint>
#include <llvm/ADT/Hashing.h>
#include <llvm/ADT/TypeSwitch.h>
#include <llvm/Support/Casting.h>
#include <mlir/Dialect/Arith/IR/Arith.h>
#include <mlir/Dialect/Func/IR/FuncOps.h>
#include <mlir/Dialect/LLVMIR/LLVMInterfaces.h>
#include <mlir/IR/Block.h>
#include <mlir/IR/BuiltinAttributes.h>
#include <mlir/IR/BuiltinOps.h>
#include <mlir/IR/BuiltinTypes.h>
#include <mlir/IR/IRMapping.h>
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

/// Creates the hash of an expression.
///
/// This hash is constructed be traversing the expression and calculating the
/// expressions' hashes as well.
static std::optional<llvm::hash_code> getExpressionHash(mlir::Value value)
{
    mlir::Operation *op = value.getDefiningOp();

    if (!op)
        return llvm::hash_combine(value.getType());

    return llvm::TypeSwitch<Operation *, std::optional<llvm::hash_code>>(op)
        .Case<arith::ConstantOp>([&](auto cst) { return llvm::hash_combine(cst.getValue()); })
        .Case<symbolic::SymOp>([&](auto sym) { return llvm::hash_combine(sym.getName()); })
        .Case<symbolic::AddOp, symbolic::SubOp, symbolic::MulOp, symbolic::DivOp>(
            [&](Operation *binop) -> std::optional<llvm::hash_code> {
                std::optional<llvm::hash_code> lhs = getExpressionHash(binop->getOperand(0));
                if (!lhs)
                    return std::nullopt;
                std::optional<llvm::hash_code> rhs = getExpressionHash(binop->getOperand(1));
                if (!rhs)
                    return std::nullopt;
                return llvm::hash_combine(binop, lhs, rhs);
            })
        .Default([](Operation *op) { return llvm::hash_combine(op->getResult(0).getType()); });
}

/// Walk the expression tree collecting values that can't be cloned
/// (LLVM::LoadOp results and block arguments) as free variables.
static void collectFreeVars(Value val, DenseSet<Value> &freeVars)
{
    Operation *op = val.getDefiningOp();

    if (!op || isa<LLVM::LoadOp>(op))
    {
        freeVars.insert(val);
        return;
    }

    for (Value operand : op->getOperands())
        collectFreeVars(operand, freeVars);
}

/// Recursively clones the expression tree into the current builder insertion
/// point, using `mapper` to deduplicate already-cloned values.
/// Free variables must already be mapped to function arguments before calling.
static Value cloneExpression(Value val, OpBuilder &builder, IRMapping &mapper)
{
    if (Value mapped = mapper.lookupOrNull(val))
        return mapped;

    Operation *op = val.getDefiningOp();

    if (!op)
        return Value();

    for (Value operand : op->getOperands())
        cloneExpression(operand, builder, mapper);

    Operation *cloned = builder.clone(*op, mapper);
    Value result = cloned->getResult(0);

    mapper.map(val, result);

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

static void createEvalFunction(PatternRewriter &rewriter, EvalOp op, SymbolRefAttr fnName, Type innerTy,
                               const DenseSet<Value> &freeVars, uint32_t evalOpHash, EvalToFuncState &state)
{
    ModuleOp module = op->getParentOfType<ModuleOp>();

    rewriter.setInsertionPointToStart(module.getBody());

    SmallVector<Type> inputTypes = {innerTy};

    for (Value fv : freeVars)
        inputTypes.push_back(fv.getType());

    FunctionType fnType = rewriter.getFunctionType(inputTypes, op.getExpr().getType());
    func::FuncOp fnOp = func::FuncOp::create(rewriter, op.getLoc(), fnName.getLeafReference(), fnType);

    fnOp.setPrivate();

    Block *fnEntryBLock = fnOp.addEntryBlock();

    rewriter.setInsertionPointToStart(fnEntryBLock);

    IRMapping mapper;

    {
        size_t i = 1;
        for (Value fv : freeVars)
            mapper.map(fv, fnEntryBLock->getArgument(i++));
    }

    Value result = cloneExpression(op.getExpr(), rewriter, mapper);

    rewriter.create<func::ReturnOp>(op.getLoc(), result);
    rewriter.setInsertionPoint(op);

    state.funcs[static_cast<int32_t>(evalOpHash)] = fnName;
}

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

        SymExprType exprTy = llvm::cast<SymExprType>(op.getExpr().getType());
        Type innerTy = exprTy.getInnerType();
        DenseSet<Value> freeVars;

        collectFreeVars(op.getExpr(), freeVars);

        uint32_t hash = static_cast<uint32_t>(*evalOpHash);
        SymbolRefAttr fnName;
        auto func = state.funcs.find(hash);

        if (func != state.funcs.end())
        {
            fnName = func->second;
        }
        else
        {
            fnName = SymbolRefAttr::get(op.getContext(), "__eval_op_" + std::to_string(hash));
            createEvalFunction(rewriter, op, fnName, innerTy, freeVars, hash, state);
        }

        SmallVector<Value> callArgs = {op.getValue()};

        callArgs.append(freeVars.begin(), freeVars.end());

        func::CallOp call = rewriter.create<func::CallOp>(op.getLoc(), fnName, op.getExpr().getType(), callArgs);

        mlir::UnrealizedConversionCastOp cast =
            UnrealizedConversionCastOp::create(rewriter, op.getLoc(), op.getResult().getType(), call.getResult(0));

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
