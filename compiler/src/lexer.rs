use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    #[token("int")]
    KwInt,
    #[token("float")]
    KwFloat,
    #[token("bool")]
    KwBool,
    #[token("string")]
    KwString,
    #[token("qubit")]
    KwQubit,
    #[token("qreg")]
    KwQreg,
    #[token("cbit")]
    KwCbit,
    #[token("if")]
    KwIf,
    #[token("else")]
    KwElse,
    #[token("while")]
    KwWhile,
    #[token("return")]
    KwReturn,
    #[token("fn")]
    KwFn,
    #[token("let")]
    KwLet,
    #[token("mut")]
    KwMut,

    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    IntLiteral(i64),
    #[regex(r"[0-9]+\.[0-9]*", |lex| lex.slice().parse().ok())]
    FloatLiteral(f64),
    #[regex(r#""([^"\\]|\\[tnr"\\])*""#, |lex| lex.slice().to_string())]
    StringLiteral(String),
    #[regex(r"\|[01]>", |lex| {
        let s = lex.slice();
        s[1..s.len()-1].parse().ok()
    })]
    QubitLiteral(i64),

    // Identifiers (including gate names and measure)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    // Operators
    #[token("=")]
    OpAssign,
    #[token("==")]
    OpEq,
    #[token("!=")]
    OpNeq,
    #[token("<")]
    OpLt,
    #[token(">")]
    OpGt,
    #[token("<=")]
    OpLe,
    #[token(">=")]
    OpGe,
    #[token("+")]
    OpAdd,
    #[token("-")]
    OpSub,
    #[token("*")]
    OpMul,
    #[token("/")]
    OpDiv,
    #[token("&")]
    OpAnd,
    #[token("|")]
    OpOr,
    #[token("^")]
    OpXor,
    #[token("!")]
    OpNot,

    // Delimiters
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("->")]
    Arrow,

    // Comments (skipped)
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", logos::skip)]
    Comment,

    // Whitespace (skipped)
    #[regex(r"\s+", logos::skip)]
    Whitespace,
}

// Add a convenience function for tokenizing
pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(source);
    
    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => {
                // Skip whitespace and comments
                if !matches!(token, Token::Whitespace | Token::Comment) {
                    tokens.push(token);
                }
            }
            Err(_) => {
                // Handle lexer errors (invalid tokens)
                eprintln!("Lexer error at position {:?}: unexpected character '{}'", 
                         lexer.span(), lexer.slice());
            }
        }
    }
    
    tokens
}