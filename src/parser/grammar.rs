use super::types::{IdentType, NumberType};

/// A program
pub struct Program {
    pub functions: Vec<FuncDeclStmt>,
}

/// All possible statements, each separated by a ";"
pub enum Statement {
    Decl(DeclStmt),
    Assing {
        var_name: String,
        value: Expression,
    },
    If {
        cond: Expression,
        then: Block,
        r#else: Option<Block>,
    },
    While {
        cond: Expression,
        block: Block,
    },
    For {},
    Return {
        rtn_expr: Expression,
    },
}

/// All possible expressions
pub enum Expression {
    BinaryOp(BinaryOpExpr),
    UnaryOp(UnaryOpExpr),
    Call(CallExpr),
    Primary(PrimaryExpr),
}

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Eq,
    EqEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
}

pub struct BinaryOpExpr {
    pub rhs: Box<Self>,
    pub op: Op,
    pub lhs: Box<Self>,
}

pub struct UnaryOpExpr {
    pub op: Op,
    pub lhs: Box<Self>,
}

pub struct CallExpr {
    pub func_name: String,
    pub params: Option<Vec<Self>>,
}

pub struct Block {
    pub stmts: Vec<Statement>,
}

pub enum PrimaryExpr {
    Number(NumberExpr),
    String(String),
    Identifier(IdentifierExpr),
}

pub struct NumberExpr {
    pub value: String,
    pub ty: NumberType,
}

pub struct IdentifierExpr {
    pub ident_name: String,
    pub ty: IdentType,
}

/// All possible declaration statements
pub enum DeclStmt {
    Let(LetStmt),
    Sym(SymStmt),
    FuncDecl(FuncDeclStmt),
    Struct(StructDeclStmt),
}

pub struct LetStmt {
    pub var_name: String,
    pub value: Option<Expression>,
}

pub struct SymStmt {
    pub var_names: Vec<SymVar>,
}

pub struct SymVar {
    pub name: String,
}

pub struct FuncDeclStmt {
    pub name: String,
    pub params: Vec<String>,
    pub body: Block,
}

pub struct StructDeclStmt {
    pub name: String,
    pub params: Vec<String>,
    pub param_ty: Vec<String>,
}

pub struct AssingStmt {
    pub var_name: String,
    pub value: Expression,
}
