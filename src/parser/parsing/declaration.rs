use crate::parser::{
    MathicParser, ParserResult,
    ast::{
        declaration::{FuncDecl, Param, VarDecl},
        statement::BlockStmt,
    },
    token::Token,
};
use crate::types::{FloatTy, MathicType, SintTy, UintTy};

impl<'a> MathicParser<'a> {
    pub fn parse_type(&self) -> ParserResult<MathicType> {
        let ident = self.consume_token(Token::Ident)?;

        let ty = match ident.lexeme {
            "i8" => MathicType::Sint(SintTy::I8),
            "i16" => MathicType::Sint(SintTy::I16),
            "i32" => MathicType::Sint(SintTy::I32),
            "i64" => MathicType::Sint(SintTy::I64),
            "i128" => MathicType::Sint(SintTy::I128),
            "u8" => MathicType::Uint(UintTy::U8),
            "u16" => MathicType::Uint(UintTy::U16),
            "u32" => MathicType::Uint(UintTy::U32),
            "u64" => MathicType::Uint(UintTy::U64),
            "u128" => MathicType::Uint(UintTy::U128),
            "f32" => MathicType::Float(FloatTy::F32),
            "f64" => MathicType::Float(FloatTy::F64),
            "bool" => MathicType::Bool,
            "void" => MathicType::Void,
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

        // Return type parsing should be here.

        let BlockStmt { stmts, .. } = self.parse_block()?;

        let span = self.merge_spans(&start_span, &self.current_span());

        Ok(FuncDecl {
            name,
            params,
            body: stmts,
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

        Ok(VarDecl { name, ty, expr })
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
