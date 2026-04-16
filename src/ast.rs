#[derive(Debug, Clone)]
pub struct Program {
    pub name: String,
    pub uses: Vec<String>,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub declarations: Vec<Decl>,
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Decl {
    Var(VarDecl),
    Proc(ProcDecl),
    Func(FuncDecl),
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct ProcDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Type,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(Assign),
    AssignIndex(AssignIndex),
    Writeln(Expr),
    Readln(Vec<String>),
    Call(CallStmt),
    Return(Option<Expr>),
    If(IfStmt),
    While(WhileStmt),
    Compound(Vec<Stmt>),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: String,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct AssignIndex {
    pub name: String,
    pub index: Expr,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct CallStmt {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub cond: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub cond: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    Str(String),
    Var(String),
    Index {
        name: String,
        index: Box<Expr>,
    },
    Call(CallExpr),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Boolean,
    String,
    Array { elem: Box<Type>, len: usize },
    Option(Box<Type>),
    Result(Box<Type>),
    Vector(Box<Type>),
    StackInt,
    MapStrStr,
    SetStr,
}
