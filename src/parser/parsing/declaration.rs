use crate::parser::{
    MathicParser, ParserResult, Span,
    ast::{
        declaration::{AstType, FuncDecl, Param, VarDecl},
        statement::BlockStmt,
    },
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_type(&self) -> ParserResult<AstType> {
        let ident = self.consume_token(Token::Ident)?;

        let ty = match ident.lexeme {
            "i8" => AstType::I8,
            "i16" => AstType::I16,
            "i32" => AstType::I32,
            "i64" => AstType::I64,
            "i128" => AstType::I128,
            "u8" => AstType::U8,
            "u16" => AstType::U16,
            "u32" => AstType::U32,
            "u64" => AstType::U64,
            "u128" => AstType::U128,
            "f32" => AstType::F32,
            "f64" => AstType::F64,
            "bool" => AstType::Bool,
            "str" => AstType::Str,
            "char" => AstType::Char,
            _ => todo!("user-defined types not yet supported: {}", ident.lexeme),
        };

        Ok(ty)
    }

    pub fn parse_func(&self) -> ParserResult<FuncDecl> {
        let start_span = self.next()?.span; // Consume Df.

        let name = {
            let ident = self.consume_token(Token::Ident)?;
            ident.lexeme.to_string()
        };

        self.consume_token(Token::LParen)?;

        let params = if self.check_next(Token::RParen)? {
            Vec::new()
        } else {
            self.parse_params()?
        };

        self.consume_token(Token::RParen)?;

        let return_ty = if self.check_next(Token::Ident)? {
            self.parse_type()?
        } else {
            AstType::Void
        };

        let BlockStmt { stmts, .. } = self.parse_block()?;

        let span = Span::from_merged_spans(start_span, self.current_span());

        Ok(FuncDecl {
            name,
            params,
            body: stmts,
            return_ty,
            span,
        })
    }

    pub fn parse_var_decl(&self) -> ParserResult<VarDecl> {
        self.next()?; // Consume Let;

        let ident = self.consume_token(Token::Ident)?;
        let name = ident.lexeme.to_string();

        self.consume_token(Token::Colon)?;
        let ty = self.parse_type()?;

        self.consume_token(Token::Eq)?;

        let expr = self.parse_expr()?;

        self.consume_token(Token::Semicolon)?;

        Ok(VarDecl {
            name,
            ty: ty.into(),
            expr,
        })
    }

    fn parse_params(&self) -> ParserResult<Vec<Param>> {
        let identifier = self.consume_token(Token::Ident)?;
        self.consume_token(Token::Colon)?;
        let ty = self.parse_type()?;

        let mut params = vec![Param {
            name: identifier.lexeme.to_string(),
            span: identifier.span,
            ty,
        }];

        while self.match_token(Token::Comma)?.is_some() {
            let identifier = self.consume_token(Token::Ident)?;
            self.consume_token(Token::Colon)?;
            let ty = self.parse_type()?;

            params.push(Param {
                name: identifier.lexeme.to_string(),
                span: identifier.span,
                ty,
            });
        }

        Ok(params)
    }
}
