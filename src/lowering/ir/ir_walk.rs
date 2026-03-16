use std::fmt;

pub mod value {
    use std::fmt::{self, Display, Formatter};

    use crate::lowering::ir::value::{ConstExpr, NumericConst, Value, ValueModifier};

    impl Display for NumericConst {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::I8(n) => write!(f, "{}", n),
                NumericConst::I16(n) => write!(f, "{}", n),
                NumericConst::I32(n) => write!(f, "{}", n),
                NumericConst::I64(n) => write!(f, "{}", n),
                NumericConst::I128(n) => write!(f, "{}", n),
                NumericConst::U8(n) => write!(f, "{}", n),
                NumericConst::U16(n) => write!(f, "{}", n),
                NumericConst::U32(n) => write!(f, "{}", n),
                NumericConst::U64(n) => write!(f, "{}", n),
                NumericConst::U128(n) => write!(f, "{}", n),
                NumericConst::F32(n) => write!(f, "{}", n),
                NumericConst::F64(n) => write!(f, "{}", n),
            }
        }
    }

    impl Display for ConstExpr {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::Str(s) => write!(f, "{}", &s),
                Self::Char(c) => write!(f, "{}", c),
                Self::Numeric(n) => write!(f, "{}", n),
                Self::Bool(b) => write!(f, "{}", b),
                Self::Void => write!(f, "void"),
            }
        }
    }

    impl Display for Value {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::InMemory {
                    local_idx,
                    modifier,
                } => match modifier {
                    Some(m) => write!(f, "%{}{}", local_idx, m),
                    None => write!(f, "%{}", local_idx),
                },
                Self::Const(c) => write!(f, "{}", c),
            }
        }
    }

    impl Display for ValueModifier {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::Field(idx) => write!(f, ".{}", idx),
            }
        }
    }
}

pub mod instructions {
    use std::fmt::{self, Display, Formatter};

    use crate::{
        lowering::ir::instruction::{InitInstruct, LValInstruct, RValInstruct, RValueKind},
        parser::ast::expression::{ArithOp, BinaryOp, CmpOp, LogicalOp, UnaryOp},
    };

    impl Display for BinaryOp {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                BinaryOp::Arithmetic(arith) => match arith {
                    ArithOp::Add => write!(f, "+"),
                    ArithOp::Sub => write!(f, "-"),
                    ArithOp::Mul => write!(f, "*"),
                    ArithOp::Div => write!(f, "/"),
                    ArithOp::Mod => write!(f, "%"),
                },
                BinaryOp::Compare(cmp) => match cmp {
                    CmpOp::Eq => write!(f, "=="),
                    CmpOp::Ne => write!(f, "!="),
                    CmpOp::Lt => write!(f, "<"),
                    CmpOp::Le => write!(f, "<="),
                    CmpOp::Gt => write!(f, ">"),
                    CmpOp::Ge => write!(f, ">="),
                },
            }
        }
    }

    impl Display for UnaryOp {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::Neg => write!(f, "-"),
                Self::Not => write!(f, "!"),
            }
        }
    }

    impl Display for LogicalOp {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                LogicalOp::And => write!(f, "and"),
                LogicalOp::Or => write!(f, "or"),
            }
        }
    }

    impl Display for RValueKind {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write_rvalue_kind(self, f, 0)
        }
    }

    pub fn write_rvalue_kind<W: std::fmt::Write>(
        kind: &RValueKind,
        f: &mut W,
        indent: usize,
    ) -> std::fmt::Result {
        let inner_indent = " ".repeat(indent + 4);
        match kind {
            RValueKind::Use { value, .. } => write!(f, "{}", value),
            RValueKind::Init {
                init_inst: InitInstruct::StructInit { fields, .. },
                ..
            } => {
                let indent = " ".repeat(indent);
                writeln!(f, "struct {{")?;

                for (i, field) in fields.iter().enumerate() {
                    writeln!(f, "{}%{}: {}", inner_indent, i, field)?;
                }

                write!(f, "{}}}", indent)
            }
            RValueKind::Binary { op, lhs, rhs, .. } => write!(f, "{} {} {}", lhs, op, rhs),
            RValueKind::Unary { op, rhs, .. } => write!(f, "{}{}", op, rhs),
            RValueKind::Logical { op, lhs, rhs, .. } => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }

    impl Display for RValInstruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.kind)
        }
    }

    pub fn write_rval_instruct<W: std::fmt::Write>(
        inst: &RValInstruct,
        f: &mut W,
        indent: usize,
    ) -> std::fmt::Result {
        write_rvalue_kind(&inst.kind, f, indent)
    }

    impl Display for LValInstruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write_lval_instruct(self, f, 0)
        }
    }

    pub fn write_lval_instruct<W: std::fmt::Write>(
        inst: &LValInstruct,
        f: &mut W,
        indent: usize,
    ) -> std::fmt::Result {
        let inner_indent = " ".repeat(indent);
        match inst {
            LValInstruct::Let {
                local_idx, init, ..
            } => {
                write!(f, "{}let %{} = ", inner_indent, local_idx)?;
                write_rval_instruct(init, f, indent)
            }
            LValInstruct::Assign {
                local_idx,
                value,
                modifier,
                ..
            } => {
                write!(f, "{}%{}", inner_indent, local_idx)?;
                if let Some(m) = modifier {
                    write!(f, "{}", m)?;
                }
                write!(f, " = ")?;
                write_rval_instruct(value, f, indent)
            }
        }
    }
}

pub mod basic_block {
    use std::fmt::{self, Display, Formatter};

    use crate::lowering::ir::{
        basic_block::{BasicBlock, Terminator},
        ir_walk::instructions::write_lval_instruct,
    };

    pub fn write_block_ir<W: std::fmt::Write>(
        block: &BasicBlock,
        f: &mut W,
        indent: usize,
    ) -> std::fmt::Result {
        let inner_indent = " ".repeat(indent + 4);

        writeln!(f, "{}block{}: {{", " ".repeat(indent), block.id)?;
        for inst in &block.instructions {
            write_lval_instruct(inst, f, indent + 4)?;
            writeln!(f)?;
        }
        writeln!(f, "{}{}", inner_indent, block.terminator)?;
        writeln!(f, "{}}}", " ".repeat(indent))
    }

    impl Display for Terminator {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::Return(Some(v), _) => write!(f, "return {}", v),
                Self::Return(None, _) => write!(f, "return"),
                Self::Branch { target, .. } => {
                    write!(f, "br block{} ", target)
                }
                Self::CondBranch {
                    condition,
                    true_block,
                    false_block,
                    ..
                } => {
                    write!(
                        f,
                        "cond_br ({}) then block{} else block{}",
                        condition, true_block, false_block
                    )
                }
                Self::Unreachable(_) => write!(f, "unreachable"),
                Self::Call {
                    callee,
                    args,
                    return_dest,
                    dest_block,
                    ..
                } => {
                    let args_str = args
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");

                    write!(
                        f,
                        "{} = call {}({}) block{}",
                        return_dest, callee, args_str, dest_block
                    )
                }
            }
        }
    }

    impl Display for BasicBlock {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write_block_ir(self, f, 0)
        }
    }
}

pub mod types {
    use std::fmt;

    use crate::lowering::ir::types::{FloatTy, MathicType, SintTy, UintTy};

    impl fmt::Display for UintTy {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                UintTy::U8 => write!(f, "u8"),
                UintTy::U16 => write!(f, "u16"),
                UintTy::U32 => write!(f, "u32"),
                UintTy::U64 => write!(f, "u64"),
                UintTy::U128 => write!(f, "u128"),
            }
        }
    }

    impl fmt::Display for SintTy {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SintTy::I8 => write!(f, "i8"),
                SintTy::I16 => write!(f, "i16"),
                SintTy::I32 => write!(f, "i32"),
                SintTy::I64 => write!(f, "i64"),
                SintTy::I128 => write!(f, "i128"),
            }
        }
    }

    impl fmt::Display for FloatTy {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                FloatTy::F32 => write!(f, "f32"),
                FloatTy::F64 => write!(f, "f64"),
            }
        }
    }

    impl fmt::Display for MathicType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MathicType::Uint(ty) => write!(f, "{}", ty),
                MathicType::Sint(ty) => write!(f, "{}", ty),
                MathicType::Float(ty) => write!(f, "{}", ty),
                MathicType::Bool => write!(f, "bool"),
                MathicType::Str => write!(f, "str"),
                MathicType::Char => write!(f, "char"),
                MathicType::Void => write!(f, "void"),
                MathicType::Adt { index, .. } => write!(f, "Adt({index})"),
            }
        }
    }
}

pub mod adts {
    use crate::lowering::ir::adts::Adt;

    pub fn write_adt_ir<W: std::fmt::Write>(
        adt: &Adt,
        f: &mut W,
        indent: usize,
    ) -> std::fmt::Result {
        let indent_str = " ".repeat(indent);

        match adt {
            Adt::Struct(s) => {
                writeln!(f, "{}struct {} {{", indent_str, s.name)?;

                for field in &s.fields {
                    writeln!(f, "{}    {}: {},", indent_str, field.name, field.ty)?;
                }

                writeln!(f, "{}}}\n", indent_str)
            }
        }
    }
}

pub mod function {
    use crate::lowering::ir::ir_walk::{adts, basic_block::write_block_ir};

    pub fn write_function_ir<W: std::fmt::Write>(
        func: &crate::lowering::ir::function::Function,
        f: &mut W,
        indent: usize,
    ) -> std::fmt::Result {
        use crate::lowering::ir::function::LocalKind;

        let indent_str = " ".repeat(indent);

        let params = func
            .sym_table
            .locals
            .iter()
            .filter(|local| matches!(local.kind, LocalKind::Param))
            .map(|p| format!("%{}", p.local_idx))
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(f, "{}df {}({}) -> i64 {{", indent_str, func.name, params)?;

        for nested_adt in func.sym_table.adts.iter() {
            adts::write_adt_ir(nested_adt, f, indent + 4)?;
        }

        for (_, nested_func) in func.sym_table.functions.iter() {
            write_function_ir(nested_func, f, indent + 4)?;
        }

        for block in func.basic_blocks.iter() {
            write_block_ir(block, f, indent + 4)?;
        }

        writeln!(f, "{}}}\n", indent_str)
    }
}
impl fmt::Display for crate::lowering::ir::Ir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for adt in &self.adts {
            adts::write_adt_ir(adt, f, 0)?;
        }

        for func in &self.functions {
            function::write_function_ir(func, f, 0)?;
        }

        Ok(())
    }
}
