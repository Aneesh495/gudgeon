//! LOLCODE 1.2 Recursive Descent Parser
//! Translates token sequences into an Abstract Syntax Tree (AST).

use crate::lol::ast::*;
use crate::lol::lexer::{Token, TokenKind};
use crate::lol::value::LolValue;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.peek().kind
    }

    fn advance(&mut self) -> &Token {
        let p = self.pos;
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        &self.tokens[p]
    }

    fn match_token(&mut self, expected: &TokenKind) -> bool {
        if self.peek_kind() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_newlines(&mut self) {
        while self.peek_kind() == &TokenKind::Newline {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        self.skip_newlines();

        // Optional HAI version
        let mut version = None;
        if self.peek_kind() == &TokenKind::Hai {
            self.advance();
            if let TokenKind::NumbarLit(v) = self.peek_kind() {
                version = Some(v.to_string());
                self.advance();
            } else if let TokenKind::NumbrLit(v) = self.peek_kind() {
                version = Some(v.to_string());
                self.advance();
            }
            self.skip_newlines();
        }

        let mut statements = Vec::new();
        while self.peek_kind() != &TokenKind::Kthxbye && self.peek_kind() != &TokenKind::Eof {
            self.skip_newlines();
            if self.peek_kind() == &TokenKind::Kthxbye || self.peek_kind() == &TokenKind::Eof {
                break;
            }
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.skip_newlines();
        }

        if self.peek_kind() == &TokenKind::Kthxbye {
            self.advance();
        }

        Ok(Program {
            version,
            statements,
        })
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.peek_kind() {
            TokenKind::CanHas => {
                self.advance();
                // Skip library name e.g. STDIO?
                if let TokenKind::Ident(_) = self.peek_kind() {
                    self.advance();
                }
                Ok(Stmt::ExprStmt(Expr::Lit(LolValue::Troof(true))))
            }
            TokenKind::IHasA => {
                self.advance();
                let name = match self.advance().kind.clone() {
                    TokenKind::Ident(s) => s,
                    other => return Err(format!("Expected identifier after I HAS A, found {:?}", other)),
                };
                let mut init = None;
                if self.peek_kind() == &TokenKind::Itz {
                    self.advance();
                    init = Some(self.parse_expression()?);
                }
                Ok(Stmt::VarDecl { name, init })
            }
            TokenKind::Visible => {
                self.advance();
                let mut args = Vec::new();
                let mut newline = true;
                args.push(self.parse_expression()?);
                while self.peek_kind() != &TokenKind::Newline
                    && self.peek_kind() != &TokenKind::Eof
                    && self.peek_kind() != &TokenKind::Bang
                {
                    if self.peek_kind() == &TokenKind::An {
                        self.advance();
                    }
                    if self.peek_kind() == &TokenKind::Bang {
                        newline = false;
                        self.advance();
                        break;
                    }
                    if self.peek_kind() == &TokenKind::Newline || self.peek_kind() == &TokenKind::Eof {
                        break;
                    }
                    args.push(self.parse_expression()?);
                }
                if self.peek_kind() == &TokenKind::Bang {
                    newline = false;
                    self.advance();
                }
                Ok(Stmt::Print { args, newline })
            }
            TokenKind::Gimmeh => {
                self.advance();
                let name = match self.advance().kind.clone() {
                    TokenKind::Ident(s) => s,
                    other => return Err(format!("Expected identifier after GIMMEH, found {:?}", other)),
                };
                Ok(Stmt::Input { name })
            }
            TokenKind::HowIzI => {
                self.advance();
                let name = match self.advance().kind.clone() {
                    TokenKind::Ident(s) => s,
                    other => return Err(format!("Expected function name after HOW IZ I, found {:?}", other)),
                };
                let mut params = Vec::new();
                while self.peek_kind() == &TokenKind::Yr || self.peek_kind() == &TokenKind::An {
                    if self.peek_kind() == &TokenKind::An {
                        self.advance();
                    }
                    if self.peek_kind() == &TokenKind::Yr {
                        self.advance();
                        if let TokenKind::Ident(p) = self.advance().kind.clone() {
                            params.push(p);
                        }
                    }
                }
                self.skip_newlines();
                let mut body = Vec::new();
                while self.peek_kind() != &TokenKind::IfUSaySo && self.peek_kind() != &TokenKind::Eof {
                    self.skip_newlines();
                    if self.peek_kind() == &TokenKind::IfUSaySo || self.peek_kind() == &TokenKind::Eof {
                        break;
                    }
                    body.push(self.parse_statement()?);
                    self.skip_newlines();
                }
                if self.peek_kind() == &TokenKind::IfUSaySo {
                    self.advance();
                }
                Ok(Stmt::FuncDef { name, params, body })
            }
            TokenKind::FoundYr => {
                self.advance();
                let val = self.parse_expression()?;
                Ok(Stmt::Return { val: Some(val) })
            }
            TokenKind::Gtfo => {
                self.advance();
                Ok(Stmt::Break)
            }
            TokenKind::ImInYr => {
                self.advance();
                let label = match self.advance().kind.clone() {
                    TokenKind::Ident(s) => s,
                    other => return Err(format!("Expected loop label after IM IN YR, found {:?}", other)),
                };
                let mut op = None;
                let mut var = None;
                if self.peek_kind() == &TokenKind::Uppin {
                    op = Some(LoopOp::Uppin);
                    self.advance();
                    if self.peek_kind() == &TokenKind::Yr {
                        self.advance();
                        if let TokenKind::Ident(v) = self.advance().kind.clone() {
                            var = Some(v);
                        }
                    }
                } else if self.peek_kind() == &TokenKind::Nerfin {
                    op = Some(LoopOp::Nerfin);
                    self.advance();
                    if self.peek_kind() == &TokenKind::Yr {
                        self.advance();
                        if let TokenKind::Ident(v) = self.advance().kind.clone() {
                            var = Some(v);
                        }
                    }
                }
                let mut cond = None;
                if self.peek_kind() == &TokenKind::Til {
                    self.advance();
                    let expr = self.parse_expression()?;
                    cond = Some(LoopCond {
                        kind: LoopCondKind::Til,
                        expr,
                    });
                } else if self.peek_kind() == &TokenKind::Wile {
                    self.advance();
                    let expr = self.parse_expression()?;
                    cond = Some(LoopCond {
                        kind: LoopCondKind::Wile,
                        expr,
                    });
                }
                self.skip_newlines();
                let mut body = Vec::new();
                while self.peek_kind() != &TokenKind::ImOuttaYr && self.peek_kind() != &TokenKind::Eof {
                    self.skip_newlines();
                    if self.peek_kind() == &TokenKind::ImOuttaYr || self.peek_kind() == &TokenKind::Eof {
                        break;
                    }
                    body.push(self.parse_statement()?);
                    self.skip_newlines();
                }
                if self.peek_kind() == &TokenKind::ImOuttaYr {
                    self.advance();
                    // Optional label check
                    if let TokenKind::Ident(_) = self.peek_kind() {
                        self.advance();
                    }
                }
                Ok(Stmt::Loop {
                    label,
                    op,
                    var,
                    cond,
                    body,
                })
            }
            TokenKind::ORly => {
                self.advance();
                self.skip_newlines();
                let mut then_branch = Vec::new();
                let mut mebbe_branches = Vec::new();
                let mut else_branch = None;

                if self.peek_kind() == &TokenKind::YaRly {
                    self.advance();
                    self.skip_newlines();
                    while self.peek_kind() != &TokenKind::Mebbe
                        && self.peek_kind() != &TokenKind::NoWai
                        && self.peek_kind() != &TokenKind::Oic
                        && self.peek_kind() != &TokenKind::Eof
                    {
                        self.skip_newlines();
                        if self.peek_kind() == &TokenKind::Mebbe
                            || self.peek_kind() == &TokenKind::NoWai
                            || self.peek_kind() == &TokenKind::Oic
                            || self.peek_kind() == &TokenKind::Eof
                        {
                            break;
                        }
                        then_branch.push(self.parse_statement()?);
                        self.skip_newlines();
                    }
                }

                while self.peek_kind() == &TokenKind::Mebbe {
                    self.advance();
                    let cond_expr = self.parse_expression()?;
                    self.skip_newlines();
                    let mut m_body = Vec::new();
                    while self.peek_kind() != &TokenKind::Mebbe
                        && self.peek_kind() != &TokenKind::NoWai
                        && self.peek_kind() != &TokenKind::Oic
                        && self.peek_kind() != &TokenKind::Eof
                    {
                        self.skip_newlines();
                        if self.peek_kind() == &TokenKind::Mebbe
                            || self.peek_kind() == &TokenKind::NoWai
                            || self.peek_kind() == &TokenKind::Oic
                            || self.peek_kind() == &TokenKind::Eof
                        {
                            break;
                        }
                        m_body.push(self.parse_statement()?);
                        self.skip_newlines();
                    }
                    mebbe_branches.push((cond_expr, m_body));
                }

                if self.peek_kind() == &TokenKind::NoWai {
                    self.advance();
                    self.skip_newlines();
                    let mut e_body = Vec::new();
                    while self.peek_kind() != &TokenKind::Oic && self.peek_kind() != &TokenKind::Eof {
                        self.skip_newlines();
                        if self.peek_kind() == &TokenKind::Oic || self.peek_kind() == &TokenKind::Eof {
                            break;
                        }
                        e_body.push(self.parse_statement()?);
                        self.skip_newlines();
                    }
                    else_branch = Some(e_body);
                }

                if self.peek_kind() == &TokenKind::Oic {
                    self.advance();
                }

                Ok(Stmt::IfThen {
                    cond: None, // Uses IT implicit variable in LOLCODE
                    then_branch,
                    mebbe_branches,
                    else_branch,
                })
            }
            TokenKind::Wtf => {
                self.advance();
                self.skip_newlines();
                let mut cases = Vec::new();
                let mut default = None;

                while self.peek_kind() == &TokenKind::Omg {
                    self.advance();
                    let val = match self.parse_literal()? {
                        Expr::Lit(v) => v,
                        _ => LolValue::Noob,
                    };
                    self.skip_newlines();
                    let mut case_body = Vec::new();
                    while self.peek_kind() != &TokenKind::Omg
                        && self.peek_kind() != &TokenKind::Omgwtf
                        && self.peek_kind() != &TokenKind::Oic
                        && self.peek_kind() != &TokenKind::Eof
                    {
                        self.skip_newlines();
                        if self.peek_kind() == &TokenKind::Omg
                            || self.peek_kind() == &TokenKind::Omgwtf
                            || self.peek_kind() == &TokenKind::Oic
                            || self.peek_kind() == &TokenKind::Eof
                        {
                            break;
                        }
                        case_body.push(self.parse_statement()?);
                        self.skip_newlines();
                    }
                    cases.push((val, case_body));
                }

                if self.peek_kind() == &TokenKind::Omgwtf {
                    self.advance();
                    self.skip_newlines();
                    let mut def_body = Vec::new();
                    while self.peek_kind() != &TokenKind::Oic && self.peek_kind() != &TokenKind::Eof {
                        self.skip_newlines();
                        if self.peek_kind() == &TokenKind::Oic || self.peek_kind() == &TokenKind::Eof {
                            break;
                        }
                        def_body.push(self.parse_statement()?);
                        self.skip_newlines();
                    }
                    default = Some(def_body);
                }

                if self.peek_kind() == &TokenKind::Oic {
                    self.advance();
                }

                Ok(Stmt::Switch { cases, default })
            }
            TokenKind::Ident(name) => {
                let id = name.clone();
                let next = self.tokens.get(self.pos + 1).map(|t| &t.kind);
                if next == Some(&TokenKind::R) {
                    self.advance(); // consume ident
                    self.advance(); // consume R
                    let value = self.parse_expression()?;
                    Ok(Stmt::Assignment { name: id, value })
                } else {
                    let expr = self.parse_expression()?;
                    Ok(Stmt::ExprStmt(expr))
                }
            }
            _ => {
                let expr = self.parse_expression()?;
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        match self.peek_kind() {
            TokenKind::SumOf => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::Sum, Box::new(left), Box::new(right)))
            }
            TokenKind::DiffOf => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::Diff, Box::new(left), Box::new(right)))
            }
            TokenKind::ProduktOf => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::Produkt, Box::new(left), Box::new(right)))
            }
            TokenKind::QuoshuntOf => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::Quoshunt, Box::new(left), Box::new(right)))
            }
            TokenKind::ModOf => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::Mod, Box::new(left), Box::new(right)))
            }
            TokenKind::BiggrOf => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::Biggr, Box::new(left), Box::new(right)))
            }
            TokenKind::SmallrOf => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::Smallr, Box::new(left), Box::new(right)))
            }
            TokenKind::BothOf => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::Both, Box::new(left), Box::new(right)))
            }
            TokenKind::EitherOf => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::Either, Box::new(left), Box::new(right)))
            }
            TokenKind::WonOf => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::Won, Box::new(left), Box::new(right)))
            }
            TokenKind::BothSaem => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::BothSaem, Box::new(left), Box::new(right)))
            }
            TokenKind::Diffrint => {
                self.advance();
                let left = self.parse_expression()?;
                self.match_token(&TokenKind::An);
                let right = self.parse_expression()?;
                Ok(Expr::Binary(BinaryOp::Diffrint, Box::new(left), Box::new(right)))
            }
            TokenKind::Not => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Expr::Unary(UnaryOp::Not, Box::new(expr)))
            }
            TokenKind::AllOf => {
                self.advance();
                let mut exprs = Vec::new();
                exprs.push(self.parse_expression()?);
                while self.match_token(&TokenKind::An) {
                    exprs.push(self.parse_expression()?);
                }
                self.match_token(&TokenKind::Mkay);
                Ok(Expr::Nary(NaryOp::All, exprs))
            }
            TokenKind::AnyOf => {
                self.advance();
                let mut exprs = Vec::new();
                exprs.push(self.parse_expression()?);
                while self.match_token(&TokenKind::An) {
                    exprs.push(self.parse_expression()?);
                }
                self.match_token(&TokenKind::Mkay);
                Ok(Expr::Nary(NaryOp::Any, exprs))
            }
            TokenKind::Smoosh => {
                self.advance();
                let mut exprs = Vec::new();
                exprs.push(self.parse_expression()?);
                while self.match_token(&TokenKind::An) {
                    exprs.push(self.parse_expression()?);
                }
                self.match_token(&TokenKind::Mkay);
                Ok(Expr::Nary(NaryOp::Smoosh, exprs))
            }
            TokenKind::Maek => {
                self.advance();
                let expr = self.parse_expression()?;
                self.match_token(&TokenKind::A);
                let target_type = match self.advance().kind.clone() {
                    TokenKind::Ident(t) => t,
                    other => format!("{:?}", other),
                };
                Ok(Expr::Cast {
                    expr: Box::new(expr),
                    target_type,
                })
            }
            TokenKind::Ident(name) => {
                let id = name.clone();
                self.advance();
                // Check if this is a function call: I IZ <func> or function call without IZ
                let mut is_call = false;
                if id == "I" && self.peek_kind() == &TokenKind::Ident("IZ".to_string()) {
                    is_call = true;
                    self.advance(); // consume IZ
                }

                if is_call {
                    let func_name = match self.advance().kind.clone() {
                        TokenKind::Ident(f) => f,
                        other => return Err(format!("Expected function name in call, found {:?}", other)),
                    };
                    let mut args = Vec::new();
                    while self.peek_kind() == &TokenKind::Yr || self.peek_kind() == &TokenKind::An {
                        if self.peek_kind() == &TokenKind::An {
                            self.advance();
                        }
                        if self.peek_kind() == &TokenKind::Yr {
                            self.advance();
                            args.push(self.parse_expression()?);
                        }
                    }
                    self.match_token(&TokenKind::Mkay);
                    Ok(Expr::Call {
                        name: func_name,
                        args,
                    })
                } else if self.peek_kind() == &TokenKind::Yr {
                    // Short form function call: <func_name> YR <arg1> AN YR <arg2>
                    let mut args = Vec::new();
                    while self.peek_kind() == &TokenKind::Yr || self.peek_kind() == &TokenKind::An {
                        if self.peek_kind() == &TokenKind::An {
                            self.advance();
                        }
                        if self.peek_kind() == &TokenKind::Yr {
                            self.advance();
                            args.push(self.parse_expression()?);
                        }
                    }
                    self.match_token(&TokenKind::Mkay);
                    Ok(Expr::Call { name: id, args })
                } else {
                    Ok(Expr::Var(id))
                }
            }
            _ => self.parse_literal(),
        }
    }

    fn parse_literal(&mut self) -> Result<Expr, String> {
        let tok = self.advance().clone();
        match tok.kind {
            TokenKind::NumbrLit(n) => Ok(Expr::Lit(LolValue::Numbr(n))),
            TokenKind::NumbarLit(f) => Ok(Expr::Lit(LolValue::Numbar(f))),
            TokenKind::YarnLit(s) => Ok(Expr::Lit(LolValue::Yarn(s))),
            TokenKind::Win => Ok(Expr::Lit(LolValue::Troof(true))),
            TokenKind::Fail => Ok(Expr::Lit(LolValue::Troof(false))),
            TokenKind::Noob => Ok(Expr::Lit(LolValue::Noob)),
            other => Err(format!("Expected expression at line {}, found {:?}", tok.line, other)),
        }
    }
}
