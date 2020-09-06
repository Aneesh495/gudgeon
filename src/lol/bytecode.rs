//! LOLCODE Bytecode Representation and Instruction Set

use crate::lol::value::LolValue;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    Halt = 0,
    PushConst = 1,
    LoadVar = 2,
    StoreVar = 3,
    Pop = 4,
    Dup = 5,

    Add = 10,
    Sub = 11,
    Mul = 12,
    Div = 13,
    Mod = 14,
    Min = 15,
    Max = 16,

    And = 20,
    Or = 21,
    Xor = 22,
    Not = 23,
    Equal = 24,
    NotEqual = 25,
    Greater = 26,
    Less = 27,

    Jump = 30,
    JumpIfFalse = 31,
    JumpIfTrue = 32,

    Call = 40,
    Return = 41,
    CallNative = 42,

    Print = 50,
    Input = 51,
    Cast = 52,
    Smoosh = 53,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    pub opcode: OpCode,
    pub operand: u32,
    pub line: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<LolValue>,
    pub var_names: Vec<String>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit(&mut self, opcode: OpCode, operand: u32, line: usize) -> usize {
        let idx = self.instructions.len();
        self.instructions.push(Instruction { opcode, operand, line });
        idx
    }

    pub fn add_constant(&mut self, value: LolValue) -> u32 {
        for (i, c) in self.constants.iter().enumerate() {
            if c == &value {
                return i as u32;
            }
        }
        let idx = self.constants.len() as u32;
        self.constants.push(value);
        idx
    }

    pub fn add_var(&mut self, name: &str) -> u32 {
        for (i, v) in self.var_names.iter().enumerate() {
            if v == name {
                return i as u32;
            }
        }
        let idx = self.var_names.len() as u32;
        self.var_names.push(name.to_string());
        idx
    }

    pub fn patch_jump(&mut self, offset: usize, target: usize) {
        if let Some(inst) = self.instructions.get_mut(offset) {
            inst.operand = target as u32;
        }
    }
}
