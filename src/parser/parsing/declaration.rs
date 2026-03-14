use crate::parser::{
    MathicParser, ParserResult, Span,
    ast::{
        declaration::{AstType, FuncDecl, Param, StructDecl, StructField, VarDecl},
        statement::BlockStmt,
    },
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_type(&self) -> ParserResult<AstType> {
        let ident = self.consume_token(Token::Ident)?;

        let ty = AstType::Type(ident.lexeme.to_string());

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
            Some(self.parse_type()?)
        } else {
            None
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

        let expr = self.parse_initializer()?;

        self.consume_token(Token::Semicolon)?;

        Ok(VarDecl { name, ty, expr })
    }

    pub fn parse_struct(&self) -> ParserResult<StructDecl> {
        let start_span = self.next()?.span; // Consume "struct"

        let name = {
            let ident = self.consume_token(Token::Ident)?;
            ident.lexeme.to_string()
        };

        self.consume_token(Token::LBrace)?;

        let fields = if self.check_next(Token::RBrace)? {
            Vec::new()
        } else {
            self.parse_struct_fields()?
        };

        self.consume_token(Token::RBrace)?;

        let span = Span::from_merged_spans(start_span, self.current_span());

        Ok(StructDecl { name, fields, span })
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

    fn parse_struct_fields(&self) -> ParserResult<Vec<StructField>> {
        let is_pub = self.match_token(Token::Pub)?.is_some();

        let field_name = self.consume_token(Token::Ident)?;
        self.consume_token(Token::Colon)?;
        let ty = self.parse_type()?;

        let mut fields = vec![StructField {
            name: field_name.lexeme.to_string(),
            ty,
            is_pub,
            span: field_name.span,
        }];

        while self.match_token(Token::Comma)?.is_some() {
            let is_pub = self.match_token(Token::Pub)?.is_some();

            let field_name = self.consume_token(Token::Ident)?;
            self.consume_token(Token::Colon)?;
            let ty = self.parse_type()?;

            fields.push(StructField {
                name: field_name.lexeme.to_string(),
                ty,
                is_pub,
                span: field_name.span,
            });
        }

        Ok(fields)
    }
}
