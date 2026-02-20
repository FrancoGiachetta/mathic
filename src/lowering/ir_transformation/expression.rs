use crate::{
    lowering::{
        Lowerer,
        ir::{
            basic_block::Terminator,
            function::Function,
            instruction::{BinaryOp, LValInstruct, LogicalOp, RValInstruct, UnaryOp},
            value::{ContExpr, Value},
        },
    },
    parser::{
        ast::{
            Span,
            expression::{ExprStmt, ExprStmtKind, PrimaryExpr},
        },
        token::Token,
    },
};

impl Lowerer {
    pub fn lower_expr(&self, func: &mut Function, expr: &ExprStmt) -> RValInstruct {
        match &expr.kind {
            ExprStmtKind::Primary(val) => self.lower_primary_value(func, val, expr.span.clone()),
            ExprStmtKind::BinOp { lhs, op, rhs } => self.lower_binary_op(
                func,
                lhs,
                match op {
                    Token::Plus => BinaryOp::Add,
                    Token::Minus => BinaryOp::Sub,
                    Token::Star => BinaryOp::Mul,
                    Token::Slash => BinaryOp::Div,
                    _ => panic!("Invalid token"),
                },
                rhs,
                expr.span.clone(),
            ),
            ExprStmtKind::Unary { op, rhs } => self.lower_unary_op(
                func,
                match op {
                    Token::Minus => UnaryOp::Neg,
                    Token::Bang => UnaryOp::Not,
                    _ => panic!("Invalid token"),
                },
                rhs,
                expr.span.clone(),
            ),
            ExprStmtKind::Group(expr) => self.lower_expr(func, expr),
            ExprStmtKind::Call { callee, args } => {
                self.lower_call(func, callee.clone(), args, expr.span.clone())
            }
            ExprStmtKind::Assign {
                name,
                expr: assign_expr,
            } => self.lower_assignment(func, name, assign_expr, expr.span.clone()),
            ExprStmtKind::Logical { lhs, op, rhs } => self.lower_logical_op(
                func,
                lhs,
                match op {
                    Token::EqEq => LogicalOp::Eq,
                    Token::BangEq => LogicalOp::Ne,
                    Token::Less => LogicalOp::Lt,
                    Token::EqLess => LogicalOp::Le,
                    Token::Greater => LogicalOp::Gt,
                    Token::EqGrater => LogicalOp::Ge,
                    _ => panic!("Invalid token"),
                },
                rhs,
                expr.span.clone(),
            ),
            ExprStmtKind::Index { .. } => todo!(),
        }
    }

    fn lower_assignment(
        &self,
        func: &mut Function,
        name: &str,
        expr: &ExprStmt,
        span: Span,
    ) -> RValInstruct {
        let Some(local_idx) = func.get_local_idx_from_name(name) else {
            panic!("variable is not declared");
        };
        let value = self.lower_expr(func, expr);

        func.last_basic_block()
            .instructions
            .push(LValInstruct::Assign {
                local_idx,
                value,
                span: Some(span),
            });

        RValInstruct::Use(Value::Const(ContExpr::Void), None)
    }

    fn lower_call(
        &self,
        func: &mut Function,
        callee: String,
        args: &[ExprStmt],
        span: Span,
    ) -> RValInstruct {
        let args: Vec<RValInstruct> = args.iter().map(|arg| self.lower_expr(func, arg)).collect();

        let local_idx = func.add_local_temp();

        func.last_basic_block().terminator = Terminator::Call {
            callee,
            args,
            span: Some(span),
            return_dest: Value::InMemory(local_idx),
            dest_block: func.last_block_idx(),
        };

        RValInstruct::Use(Value::InMemory(local_idx), None)
    }

    fn lower_binary_op(
        &self,
        func: &mut Function,
        lhs: &ExprStmt,
        op: BinaryOp,
        rhs: &ExprStmt,
        span: Span,
    ) -> RValInstruct {
        let lhs = self.lower_expr(func, lhs).into();
        let rhs = self.lower_expr(func, rhs).into();

        RValInstruct::Binary {
            op,
            lhs,
            rhs,
            span: Some(span),
        }
    }

    fn lower_logical_op(
        &self,
        func: &mut Function,
        lhs: &ExprStmt,
        op: LogicalOp,
        rhs: &ExprStmt,
        span: Span,
    ) -> RValInstruct {
        let lhs = self.lower_expr(func, lhs).into();
        let rhs = self.lower_expr(func, rhs).into();

        RValInstruct::Logical {
            op,
            lhs,
            rhs,
            span: Some(span),
        }
    }

    fn lower_unary_op(
        &self,
        func: &mut Function,
        op: UnaryOp,
        rhs: &ExprStmt,
        span: Span,
    ) -> RValInstruct {
        let rhs = self.lower_expr(func, rhs).into();

        RValInstruct::Unary {
            op,
            operand: rhs,
            span: Some(span),
        }
    }

    fn lower_primary_value(
        &self,
        func: &mut Function,
        expr: &PrimaryExpr,
        span: Span,
    ) -> RValInstruct {
        let value = match expr {
            PrimaryExpr::Ident(name) => {
                if let Some(idx) = func.get_local_idx_from_name(name) {
                    Value::InMemory(idx)
                } else {
                    panic!("variable is not declared");
                }
            }
            PrimaryExpr::Num(n) => Value::Const(ContExpr::Int(n.clone())),
            PrimaryExpr::Bool(b) => Value::Const(ContExpr::Bool(*b)),
            PrimaryExpr::Str(_) => todo!(),
        };

        RValInstruct::Use(value, Some(span))
    }
}
