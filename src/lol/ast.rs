//! LOLCODE Abstract Syntax Tree (AST) definitions

use crate::lol::value::LolValue;

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Not,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Sum,
    Diff,
    Produkt,
    Quoshunt,
    Mod,
    Biggr,
    Smallr,
    Both,
    Either,
    Won,
    BothSaem,
    Diffrint,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NaryOp {
    All,
    Any,
    Smoosh,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoopOp {
    Uppin,
    Nerfin,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoopCondKind {
    Til,
    Wile,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoopCond {
    pub kind: LoopCondKind,
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Lit(LolValue),
    Var(String),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Nary(NaryOp, Vec<Expr>),
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Cast {
        expr: Box<Expr>,
        target_type: String,
    },
    ArrayLit(Vec<Expr>),
    ArrayGet {
        target: Box<Expr>,
        index: Box<Expr>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    VarDecl {
        name: String,
        init: Option<Expr>,
    },
    Assignment {
        name: String,
        value: Expr,
    },
    ArraySet {
        target: String,
        index: Expr,
        value: Expr,
    },
    Print {
        args: Vec<Expr>,
        newline: bool,
    },
    Input {
        name: String,
    },
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return {
        val: Option<Expr>,
    },
    Break,
    Loop {
        label: String,
        op: Option<LoopOp>,
        var: Option<String>,
        cond: Option<LoopCond>,
        body: Vec<Stmt>,
    },
    IfThen {
        cond: Option<Expr>,
        then_branch: Vec<Stmt>,
        mebbe_branches: Vec<(Expr, Vec<Stmt>)>,
        else_branch: Option<Vec<Stmt>>,
    },
    Switch {
        cases: Vec<(LolValue, Vec<Stmt>)>,
        default: Option<Vec<Stmt>>,
    },
    ExprStmt(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub version: Option<String>,
    pub statements: Vec<Stmt>,
}
