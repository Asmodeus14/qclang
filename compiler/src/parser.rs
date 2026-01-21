use crate::lexer::{Token, Token::*};
use crate::ast::*;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
    pub errors: Vec<String>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
            errors: Vec::new(),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut functions = Vec::new();
        
        while self.peek().is_some() {
            if let Some(func) = self.parse_function() {
                functions.push(func);
            } else {
                // Skip tokens until next function or EOF
                while let Some(_) = self.tokens.next() {
                    if self.peek() == Some(&KwFn) {
                        break;
                    }
                }
            }
        }
        
        Program { functions }
    }

    fn parse_function(&mut self) -> Option<Function> {
        self.expect(KwFn)?;
        
        let name = match self.expect_ident() {
            Some(name) => name,
            None => return None,
        };
        
        self.expect(ParenOpen)?;
        let params = self.parse_params();
        self.expect(ParenClose)?;
        
        self.expect(Arrow)?;
        let return_type = self.parse_type()?;
        
        self.expect(BraceOpen)?;
        let body = self.parse_block_statements()?;
        self.expect(BraceClose)?;
        
        Some(Function {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_params(&mut self) -> Vec<Param> {
        let mut params = Vec::new();
        
        if self.peek() == Some(&ParenClose) {
            return params;
        }
        
        loop {
            if let Some(name) = self.expect_ident() {
                self.expect(Colon);
                let ty = self.parse_type().unwrap_or(Type::Unit);
                let mutable = false;
                params.push(Param { name, ty, mutable });
                
                if !self.consume_if(Comma) {
                    break;
                }
            } else {
                break;
            }
        }
        
        params
    }

    fn parse_type(&mut self) -> Option<Type> {
        let token = self.tokens.next()?;
        match token {
            KwInt => Some(Type::Int),
            KwFloat => Some(Type::Float),
            KwBool => Some(Type::Bool),
            KwString => Some(Type::String),
            KwQubit => Some(Type::Qubit),
            KwCbit => Some(Type::Cbit),
            KwQreg => {
                self.expect(BracketOpen)?;
                let size = self.parse_int_literal()?;
                self.expect(BracketClose)?;
                Some(Type::Qreg(Box::new(Type::Qubit), size as usize))
            }
            _ => {
                // Enhanced error message
                self.errors.push(format!(
                    "Expected type (int, float, bool, string, qubit, cbit, qreg[...]), found {}",
                    self.token_to_string(&token)
                ));
                None
            }
        }
    }

    fn parse_block_statements(&mut self) -> Option<Vec<Stmt>> {
        let mut stmts = Vec::new();
        
        while self.peek() != Some(&BraceClose) && self.peek().is_some() {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                // Skip tokens until semicolon or brace
                while let Some(token) = self.tokens.next() {
                    if token == Semicolon || token == BraceClose {
                        if token == BraceClose {
                            break;
                        }
                        break;
                    }
                }
            }
        }
        
        Some(stmts)
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek() {
            Some(&KwLet) => self.parse_let_stmt(),
            Some(&KwQubit) | Some(&KwCbit) | Some(&KwInt) | Some(&KwFloat) | Some(&KwBool) | Some(&KwString) => {
                self.parse_var_decl_stmt()
            }
            Some(&KwIf) => self.parse_if_stmt(),
            Some(&KwWhile) => self.parse_while_stmt(),
            Some(&KwReturn) => self.parse_return_stmt(),
            Some(&BraceOpen) => self.parse_block_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_var_decl_stmt(&mut self) -> Option<Stmt> {
        let type_token = self.tokens.next()?;
        
        let ty = match type_token {
            KwInt => Type::Int,
            KwFloat => Type::Float,
            KwBool => Type::Bool,
            KwString => Type::String,
            KwQubit => Type::Qubit,
            KwCbit => Type::Cbit,
            _ => {
                self.errors.push(format!(
                    "Expected type in variable declaration, found {}",
                    self.token_to_string(&type_token)
                ));
                return None;
            }
        };
        
        let name = match self.expect_ident() {
            Some(name) => name,
            None => return None,
        };
        
        if !self.consume_if(OpAssign) {
            self.expect(Semicolon)?;
            return Some(Stmt::Let(name, ty, Expr::LiteralInt(0)));
        }
        
        let expr = self.parse_expr()?;
        self.expect(Semicolon)?;
        
        Some(Stmt::Let(name, ty, expr))
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        self.expect(KwLet)?;
        
        let name = self.expect_ident()?;
        self.expect(Colon);
        let ty = self.parse_type().unwrap_or(Type::Unit);
        
        self.expect(OpAssign);
        let expr = self.parse_expr()?;
        
        self.expect(Semicolon);
        
        Some(Stmt::Let(name, ty, expr))
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        let expr = self.parse_expr()?;
        self.expect(Semicolon)?;
        
        // Check if it's an assignment expression
        if let Expr::BinaryOp(ref lhs, BinaryOp::Assign, ref rhs) = &expr {
            if let Expr::Variable(var_name) = &**lhs {
                return Some(Stmt::Assign(var_name.clone(), (**rhs).clone()));
            }
        }
        
        Some(Stmt::Expr(expr))
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> Option<Expr> {
        let lhs = self.parse_or_expr()?;
        
        if self.consume_if(OpAssign) {
            let rhs = self.parse_assignment_expr()?;
            Some(Expr::BinaryOp(
                Box::new(lhs),
                BinaryOp::Assign,
                Box::new(rhs)
            ))
        } else {
            Some(lhs)
        }
    }

    fn parse_or_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_and_expr()?;
        
        while self.consume_if(OpOr) {
            let rhs = self.parse_and_expr()?;
            expr = Expr::BinaryOp(Box::new(expr), BinaryOp::Or, Box::new(rhs));
        }
        
        Some(expr)
    }

    fn parse_and_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_equality_expr()?;
        
        while self.consume_if(OpAnd) {
            let rhs = self.parse_equality_expr()?;
            expr = Expr::BinaryOp(Box::new(expr), BinaryOp::And, Box::new(rhs));
        }
        
        Some(expr)
    }

    fn parse_equality_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_relational_expr()?;
        
        while let Some(op) = self.parse_equality_op() {
            let rhs = self.parse_relational_expr()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(rhs));
        }
        
        Some(expr)
    }

    fn parse_equality_op(&mut self) -> Option<BinaryOp> {
        match self.peek() {
            Some(&OpEq) => { self.tokens.next(); Some(BinaryOp::Eq) }
            Some(&OpNeq) => { self.tokens.next(); Some(BinaryOp::Neq) }
            _ => None,
        }
    }

    fn parse_relational_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_additive_expr()?;
        
        while let Some(op) = self.parse_relational_op() {
            let rhs = self.parse_additive_expr()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(rhs));
        }
        
        Some(expr)
    }

    fn parse_relational_op(&mut self) -> Option<BinaryOp> {
        match self.peek() {
            Some(&OpLt) => { self.tokens.next(); Some(BinaryOp::Lt) }
            Some(&OpGt) => { self.tokens.next(); Some(BinaryOp::Gt) }
            Some(&OpLe) => { self.tokens.next(); Some(BinaryOp::Le) }
            Some(&OpGe) => { self.tokens.next(); Some(BinaryOp::Ge) }
            _ => None,
        }
    }

    fn parse_additive_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_multiplicative_expr()?;
        
        while let Some(op) = self.parse_additive_op() {
            let rhs = self.parse_multiplicative_expr()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(rhs));
        }
        
        Some(expr)
    }

    fn parse_additive_op(&mut self) -> Option<BinaryOp> {
        match self.peek() {
            Some(&OpAdd) => { self.tokens.next(); Some(BinaryOp::Add) }
            Some(&OpSub) => { self.tokens.next(); Some(BinaryOp::Sub) }
            _ => None,
        }
    }

    fn parse_multiplicative_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_unary_expr()?;
        
        while let Some(op) = self.parse_multiplicative_op() {
            let rhs = self.parse_unary_expr()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(rhs))
        }
        
        Some(expr)
    }

    fn parse_multiplicative_op(&mut self) -> Option<BinaryOp> {
        match self.peek() {
            Some(&OpMul) => { self.tokens.next(); Some(BinaryOp::Mul) }
            Some(&OpDiv) => { self.tokens.next(); Some(BinaryOp::Div) }
            _ => None,
        }
    }

    fn parse_unary_expr(&mut self) -> Option<Expr> {
        if let Some(op) = self.parse_unary_op() {
            let expr = self.parse_unary_expr()?;
            Some(Expr::UnaryOp(op, Box::new(expr)))
        } else {
            self.parse_primary_expr()
        }
    }

    fn parse_unary_op(&mut self) -> Option<UnaryOp> {
        match self.peek() {
            Some(&OpSub) => { self.tokens.next(); Some(UnaryOp::Neg) }
            Some(&OpNot) => { self.tokens.next(); Some(UnaryOp::Not) }
            _ => None,
        }
    }

    fn parse_primary_expr(&mut self) -> Option<Expr> {
        let token = self.tokens.next()?;
        match token {
            IntLiteral(n) => Some(Expr::LiteralInt(n)),
            FloatLiteral(f) => Some(Expr::LiteralFloat(f)),
            StringLiteral(s) => Some(Expr::LiteralString(s)),
            QubitLiteral(q) => Some(Expr::LiteralQubit(q)),
            Ident(name) => {
                if self.peek() == Some(&ParenOpen) {
                    self.tokens.next();
                    let args = self.parse_args()?;
                    self.expect(ParenClose)?;
                    
                    // Check for built-in gates
                    if name == "H" || name == "X" || name == "Y" || name == "Z" || name == "CNOT" {
                        Some(Expr::GateApply(name, args))
                    } else if name == "measure" {
                        if args.len() == 1 {
                            Some(Expr::Measure(Box::new(args[0].clone())))
                        } else {
                            self.errors.push(format!("measure expects 1 argument, got {}", args.len()));
                            None
                        }
                    } else {
                        Some(Expr::Call(name, args))
                    }
                } else {
                    Some(Expr::Variable(name))
                }
            }
            ParenOpen => {
                let expr = self.parse_expr()?;
                self.expect(ParenClose)?;
                Some(expr)
            }
            _ => {
                self.errors.push(format!(
                    "Expected primary expression (literal, identifier, or '('), found {}",
                    self.token_to_string(&token)
                ));
                None
            }
        }
    }

    fn parse_args(&mut self) -> Option<Vec<Expr>> {
        let mut args = Vec::new();
        
        if self.peek() == Some(&ParenClose) {
            return Some(args);
        }
        
        loop {
            if let Some(expr) = self.parse_expr() {
                args.push(expr);
            } else {
                break;
            }
            
            if !self.consume_if(Comma) {
                break;
            }
        }
        
        Some(args)
    }

    fn parse_int_literal(&mut self) -> Option<i64> {
        let token = self.tokens.next()?;
        match token {
            IntLiteral(n) => Some(n),
            _ => {
                self.errors.push(format!(
                    "Expected integer literal, found {}",
                    self.token_to_string(&token)
                ));
                None
            }
        }
    }

    fn parse_if_stmt(&mut self) -> Option<Stmt> {
        self.expect(KwIf)?;
        self.expect(ParenOpen)?;
        let condition = self.parse_expr()?;
        self.expect(ParenClose)?;
        
        let then_branch = Box::new(self.parse_stmt()?);
        let else_branch = if self.consume_if(KwElse) {
            Some(Box::new(self.parse_stmt()?))
        } else {
            None
        };
        
        Some(Stmt::If(condition, then_branch, else_branch))
    }

    fn parse_while_stmt(&mut self) -> Option<Stmt> {
        self.expect(KwWhile)?;
        self.expect(ParenOpen)?;
        let condition = self.parse_expr()?;
        self.expect(ParenClose)?;
        
        let body = Box::new(self.parse_stmt()?);
        Some(Stmt::While(condition, body))
    }

    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        self.expect(KwReturn)?;
        let expr = if self.peek() != Some(&Semicolon) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(Semicolon)?;
        Some(Stmt::Return(expr))
    }

    fn parse_block_stmt(&mut self) -> Option<Stmt> {
        self.expect(BraceOpen)?;
        let stmts = self.parse_block_statements()?;
        self.expect(BraceClose)?;
        Some(Stmt::Block(stmts))
    }

    fn expect_ident(&mut self) -> Option<String> {
        let token = self.tokens.next()?;
        match token {
            Ident(name) => Some(name),
            _ => {
                self.errors.push(format!(
                    "Expected identifier, found {}",
                    self.token_to_string(&token)
                ));
                None
            }
        }
    }

    fn expect(&mut self, expected: Token) -> Option<()> {
        let token = self.tokens.next()?;
        if token == expected {
            Some(())
        } else {
            self.errors.push(format!(
                "Expected {}, found {}",
                self.token_to_string(&expected),
                self.token_to_string(&token)
            ));
            None
        }
    }

    fn consume_if(&mut self, expected: Token) -> bool {
        if self.peek() == Some(&expected) {
            self.tokens.next();
            true
        } else {
            false
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    // Helper function to convert token to string for error messages
    fn token_to_string(&self, token: &Token) -> String {
        match token {
            KwInt => "int".to_string(),
            KwFloat => "float".to_string(),
            KwBool => "bool".to_string(),
            KwString => "string".to_string(),
            KwQubit => "qubit".to_string(),
            KwQreg => "qreg".to_string(),
            KwCbit => "cbit".to_string(),
            KwIf => "if".to_string(),
            KwElse => "else".to_string(),
            KwWhile => "while".to_string(),
            KwReturn => "return".to_string(),
            KwFn => "fn".to_string(),
            KwLet => "let".to_string(),
            KwMut => "mut".to_string(),
            IntLiteral(n) => format!("integer '{}'", n),
            FloatLiteral(f) => format!("float '{}'", f),
            StringLiteral(s) => format!("string \"{}\"", s),
            QubitLiteral(q) => format!("qubit literal '|{}>'", q),
            Ident(name) => format!("identifier '{}'", name),
            OpAssign => "'='".to_string(),
            OpEq => "'=='".to_string(),
            OpNeq => "'!='".to_string(),
            OpLt => "'<'".to_string(),
            OpGt => "'>'".to_string(),
            OpLe => "'<='".to_string(),
            OpGe => "'>='".to_string(),
            OpAdd => "'+'".to_string(),
            OpSub => "'-'".to_string(),
            OpMul => "'*'".to_string(),
            OpDiv => "'/'".to_string(),
            OpAnd => "'&'".to_string(),
            OpOr => "'|'".to_string(),
            OpXor => "'^'".to_string(),
            OpNot => "'!'".to_string(),
            ParenOpen => "'('".to_string(),
            ParenClose => "')'".to_string(),
            BraceOpen => "'{{'".to_string(),
            BraceClose => "'}}'".to_string(),
            BracketOpen => "'['".to_string(),
            BracketClose => "']'".to_string(),
            Comma => "','".to_string(),
            Colon => "':'".to_string(),
            Semicolon => "';'".to_string(),
            Arrow => "'->'".to_string(),
            Whitespace => "whitespace".to_string(),
            Comment => "comment".to_string(),
            Error => "error token".to_string(),
        }
    }
}