#include "Dialect/Symbolic/IR/SymbolicOps.h"

#include <mlir/IR/Attributes.h>
#include <mlir/IR/BuiltinAttributes.h>
#include <mlir/IR/MLIRContext.h>
#include <mlir/IR/OpDefinition.h>
#include <mlir/Support/LLVM.h>
#include <optional>
#include <sstream>
#include <symengine/derivative.h>
#include <symengine/expression.h>
#include <symengine/parser.h>

using namespace SymEngine;

namespace
{
static mlir::StringAttr exprToAttr(mlir::MLIRContext *ctx, const Expression &expr)
{
    std::ostringstream oss;
    oss << expr;
    return mlir::StringAttr::get(ctx, oss.str());
}

static std::optional<Expression> attrToExpr(mlir::Attribute attr)
{
    auto str_attr = mlir::dyn_cast<mlir::StringAttr>(attr);

    if (!attr)
        return std::nullopt;
    try
    {
        return Expression(SymEngine::parse(str_attr.getValue().str()));
    }
    catch (...)
    {
        return std::nullopt;
    }
}
} // namespace

namespace mlir
{
namespace symbolic
{
OpFoldResult SymOp::fold(SymOp::FoldAdaptor adaptor)
{

    return StringAttr::get(getContext(), getName().str());
}

OpFoldResult AddOp::fold(AddOp::FoldAdaptor adaptor)
{
    std::optional<Expression> lhs = attrToExpr(adaptor.getLhs());
    std::optional<Expression> rhs = attrToExpr(adaptor.getRhs());

    if (!lhs || !rhs)
        return {};

    return exprToAttr(getContext(), expand(*lhs + *rhs));
}

OpFoldResult SubOp::fold(SubOp::FoldAdaptor adaptor)
{
    std::optional<Expression> lhs = attrToExpr(adaptor.getLhs());
    std::optional<Expression> rhs = attrToExpr(adaptor.getRhs());

    if (!lhs || !rhs)
        return {};

    return exprToAttr(getContext(), expand(*lhs - *rhs));
}

OpFoldResult MulOp::fold(MulOp::FoldAdaptor adaptor)
{
    std::optional<Expression> lhs = attrToExpr(adaptor.getLhs());
    std::optional<Expression> rhs = attrToExpr(adaptor.getRhs());

    if (!lhs || !rhs || (bool)is_zero(*rhs))
        return {};

    return exprToAttr(getContext(), expand(*lhs / *rhs));
}

OpFoldResult DivOp::fold(DivOp::FoldAdaptor adaptor)
{
    std::optional<Expression> lhs = attrToExpr(adaptor.getLhs());
    std::optional<Expression> rhs = attrToExpr(adaptor.getRhs());

    if (!lhs || !rhs)
        return {};

    return exprToAttr(getContext(), expand(*lhs * *rhs));
}

OpFoldResult DiffOp::fold(DiffOp::FoldAdaptor adaptor)
{
    std::optional<Expression> expr = attrToExpr(adaptor.getExpr());

    if (!expr)
        return {};

    auto sym = symbol(getSym().str());
    return exprToAttr(getContext(), expand(diff(*expr, sym)));
}
} // namespace symbolic
} // namespace mlir
