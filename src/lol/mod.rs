//! LOLCODE Language Engine Subsystem
//! Implements a conforming LOLCODE 1.2+ lexer, parser, AST, bytecode compiler,
//! and virtual machine runtime with maritime navigation builtins.

pub mod ast;
pub mod bytecode;
pub mod lexer;
pub mod parser;
pub mod value;
pub mod vm;

use std::fs;
use std::path::Path;

pub use ast::Program;
pub use value::LolValue;
pub use vm::LolVM;

pub fn parse_source(source: &str) -> Result<Program, String> {
    let mut lex = lexer::Lexer::new(source);
    let tokens = lex.tokenize()?;
    let mut parse = parser::Parser::new(tokens);
    parse.parse()
}

pub fn run_source(source: &str) -> Result<(LolValue, LolVM), String> {
    let prog = parse_source(source)?;
    let mut vm = LolVM::new();
    let res = vm.run_program(&prog)?;
    Ok((res, vm))
}

pub fn run_file(path: &Path, vm: &mut LolVM) -> Result<LolValue, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    let prog = parse_source(&content)?;
    vm.run_program(&prog)
}
