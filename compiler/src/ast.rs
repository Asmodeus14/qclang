#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Qubit,
    Qreg(Box<Type>, usize),
    Cbit,
    Function(Vec<Type>, Box<Type>),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    LiteralInt(i64),
    LiteralFloat(f64),
    LiteralBool(bool),
    LiteralString(String),
    LiteralQubit(i64),
    Variable(String),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    Call(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Measure(Box<Expr>),
    GateApply(String, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
    Eq, Neq, Lt, Gt, Le, Ge,
    And, Or, Xor,
    Assign,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg, Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Let(String, Type, Expr),
    Assign(String, Expr),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Return(Option<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: Type,
    pub mutable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Type,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
}