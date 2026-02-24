use crate::{
    lowering::{
        Lowerer,
        error::LoweringError,
        ir::{
            basic_block::Terminator,
            function::{Function, LocalKind},
            instruction::{LValInstruct, RValInstruct},
            value::{ContExpr, Value},
        },
    },
    parser::ast::{
        Span,
        expression::{BinaryOp, ExprStmt, ExprStmtKind, LogicalOp, PrimaryExpr, UnaryOp},
    },
};

impl Lowerer {
    pub fn lower_expr(
        &self,
        func: &mut Function,
        expr: &ExprStmt,
    ) -> Result<RValInstruct, LoweringError> {
        match &expr.kind {
            ExprStmtKind::Primary(val) => self.lower_primary_value(func, val, expr.span.clone()),
            ExprStmtKind::Binary { lhs, op, rhs } => {
                self.lower_binary_op(func, lhs, *op, rhs, expr.span.clone())
            }
            ExprStmtKind::Unary { op, rhs } => {
                self.lower_unary_op(func, *op, rhs, expr.span.clone())
            }
            ExprStmtKind::Group(expr) => self.lower_expr(func, expr),
            ExprStmtKind::Call { callee, args } => {
                self.lower_call(func, callee.clone(), args, expr.span.clone())
            }
            ExprStmtKind::Assign {
                name,
                expr: assign_expr,
            } => self.lower_assignment(func, name, assign_expr, expr.span.clone()),
            ExprStmtKind::Logical { lhs, op, rhs } => {
                self.lower_logical_op(func, lhs, *op, rhs, expr.span.clone())
            }
            ExprStmtKind::Index { .. } => todo!(),
        }
    }

    fn lower_assignment(
        &self,
        func: &mut Function,
        name: &str,
        expr: &ExprStmt,
        span: Span,
    ) -> Result<RValInstruct, LoweringError> {
        let local_idx =
            func.get_local_idx_from_name(name)
                .ok_or(LoweringError::UndeclaredVariable {
                    name: name.to_string(),
                    span: span.clone(),
                })?;
        let value = self.lower_expr(func, expr)?;

        // FUTURE: check that value is of the same type as the local.

        func.get_basic_block_mut(func.last_block_idx())
            .instructions
            .push(LValInstruct::Assign {
                local_idx,
                value,
                span: Some(span),
            });

        Ok(RValInstruct::Use(Value::Const(ContExpr::Void), None))
    }

    fn lower_call(
        &self,
        func: &mut Function,
        callee: String,
        args: &[ExprStmt],
        span: Span,
    ) -> Result<RValInstruct, LoweringError> {
        // FUTURE: check if the function is declared or if it will be.

        let args: Vec<RValInstruct> = args
            .iter()
            .map(|arg| self.lower_expr(func, arg))
            .collect::<Result<_, _>>()?;

        // FUTURE: check that the amount of args matches the expected and that
        // every type matches the expected type.

        let local_idx = func.add_local(None, None, LocalKind::Temp)?;

        let dest_block_idx = func.last_block_idx() + 1;

        func.get_basic_block_mut(func.last_block_idx()).terminator = Terminator::Call {
            callee,
            args,
            span: Some(span),
            return_dest: Value::InMemory(local_idx),
            dest_block: dest_block_idx,
        };

        func.add_block(Terminator::Return(None, None), None);

        Ok(RValInstruct::Use(Value::InMemory(local_idx), None))
    }

    fn lower_binary_op(
        &self,
        func: &mut Function,
        lhs: &ExprStmt,
        op: BinaryOp,
        rhs: &ExprStmt,
        span: Span,
    ) -> Result<RValInstruct, LoweringError> {
        let lhs = self.lower_expr(func, lhs)?.into();
        let rhs = self.lower_expr(func, rhs)?.into();

        // FUTURE: check that both lhs and rhs are of the same numeric type.

        Ok(RValInstruct::Binary { op, lhs, rhs, span })
    }

    fn lower_logical_op(
        &self,
        func: &mut Function,
        lhs: &ExprStmt,
        op: LogicalOp,
        rhs: &ExprStmt,
        span: Span,
    ) -> Result<RValInstruct, LoweringError> {
        let lhs = self.lower_expr(func, lhs)?.into();
        let rhs = self.lower_expr(func, rhs)?.into();

        // FUTURE: check that both lhs and rhs are of type boolean.

        Ok(RValInstruct::Logical { op, lhs, rhs, span })
    }

    fn lower_unary_op(
        &self,
        func: &mut Function,
        op: UnaryOp,
        rhs: &ExprStmt,
        span: Span,
    ) -> Result<RValInstruct, LoweringError> {
        let rhs = self.lower_expr(func, rhs)?.into();

        // FUTURE: check that rhs is type of type numeric or boolean depending
        // on op.

        Ok(RValInstruct::Unary { op, rhs, span })
    }

    fn lower_primary_value(
        &self,
        func: &mut Function,
        expr: &PrimaryExpr,
        span: Span,
    ) -> Result<RValInstruct, LoweringError> {
        let value = match expr {
            PrimaryExpr::Ident(name) => Value::InMemory(func.get_local_idx_from_name(name).ok_or(
                LoweringError::UndeclaredVariable {
                    name: name.clone(),
                    span: span.clone(),
                },
            )?),
            PrimaryExpr::Num(n) => Value::Const(ContExpr::Int(n.clone())),
            PrimaryExpr::Bool(b) => Value::Const(ContExpr::Bool(*b)),
            PrimaryExpr::Str(_) => todo!(),
        };

        Ok(RValInstruct::Use(value, Some(span)))
    }
}
