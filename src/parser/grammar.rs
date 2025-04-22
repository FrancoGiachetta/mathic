pub struct Program {
    pub funcs: Vec<GrammarItem>,
}

pub enum GrammarItem {
    Stmt(StmtItem),
    Expr(ExprItem),
}

pub enum StmtItem {
    Function {
        name: String,
        // pub params: Vec<Param>,
        body: Block,
        // pub return_ty: Type,
    },
    Block(Block),
}

pub enum ExprItem {}

pub struct Block(pub Vec<StmtItem>);
